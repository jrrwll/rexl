use std::fs;
use std::ops::Deref;

use lettre::{
    Address, SmtpTransport, Transport,
    transport::smtp::authentication::Credentials,
};
use lettre::message::{
    Attachment, Mailbox, Message, MessageBuilder, MultiPart, SinglePart,
    header::ContentType,
};
use crate::mime::mime_from_filename;

pub struct MailMessageBuilder {
    message_builder: MessageBuilder,
}

macro_rules! impl_mailbox {
    ($method:ident, $with_method:ident) => {
        pub fn $method(mut self, mbox: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let mailbox: Mailbox = mbox.parse()?;
            self.message_builder = self.message_builder.$method(mailbox);
            Ok(self)
        }

        pub fn $with_method<S: Into<String>>(
            mut self, name: S, address: &str,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let addr: Address = address.parse()?;

            let mailbox = Mailbox::new(Some(name.into()), addr);
            self.message_builder = self.message_builder.$method(mailbox);
            Ok(self)
        }
    };
}

impl MailMessageBuilder {
    pub fn builder() -> Self {
        Self { message_builder: Message::builder() }
    }

    impl_mailbox!(from, with_from);

    impl_mailbox!(to, with_to);

    impl_mailbox!(cc, with_cc);

    impl_mailbox!(bcc, with_bcc);

    impl_mailbox!(sender, with_sender);

    impl_mailbox!(reply_to, with_reply_to);

    pub fn subject<S: Into<String>>(mut self, subject: S) -> Self {
        self.message_builder = self.message_builder.subject(subject);
        self
    }

    pub fn body<S: Into<String>>(
        mut self, body: S, html: bool,
    ) -> Result<MailMessage, Box<dyn std::error::Error>> {
        let part = Self::build_single_part(body, html);
        let message = self.message_builder.singlepart(part)?;
        Ok(MailMessage { message })
    }

    pub fn body_with_attachments<S: Into<String>, P: Into<String>>(
        mut self, body: S, html: bool, filenames: Vec<P>,
    ) -> Result<MailMessage, Box<dyn std::error::Error>> {
        let part = Self::build_single_part(body, html);

        let mut multipart = MultiPart::mixed().build();
        multipart = multipart.singlepart(part);

        for filename in filenames {
            let filename = filename.into();
            let content = fs::read(&filename)?;
            let mime_type = mime_from_filename(&filename);
            let content_type = ContentType::parse(mime_type.as_ref().unwrap())?;

            let attachment = Attachment::new(filename).body(content, content_type);
            multipart = multipart.singlepart(attachment);
        }

        let message = self
            .message_builder
            .multipart(multipart)
            .map_err(|e| e.to_string())?;
        Ok(MailMessage { message })
    }

    fn build_single_part<S: Into<String>>(body: S, html: bool) -> SinglePart {
        if html { SinglePart::html(body.into()) } else { SinglePart::plain(body.into()) }
    }
}

pub struct MailMessage {
    pub message: Message,
}

impl Into<Message> for MailMessage {
    fn into(self) -> Message {
        self.message
    }
}

impl Deref for MailMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

pub struct MailSender {
    transport: SmtpTransport,
}

macro_rules! impl_transport {
    ($method:ident, $transport_method:ident, $param_name:ident) => {
        pub fn $method(
            $param_name: &str, credentials: Option<(String, String)>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let mut transport_builder = SmtpTransport::$transport_method($param_name)?;
            if let Some((username, password)) = credentials {
                let creds = Credentials::new(username, password);
                transport_builder = transport_builder.credentials(creds);
            }

            let transport = transport_builder.build();
            Ok(Self { transport })
        }
    };
}

impl MailSender {
    impl_transport!(from_url, from_url, connection_url);

    // STARTTLS 587, like: smtp.gmail.com
    impl_transport!(from_starttls_relay, starttls_relay, relay);

    impl_transport!(from_relay, relay, relay);

    pub fn send(&self, message: &MailMessage) -> Result<(), Box<dyn std::error::Error>> {
        self.transport.send(message)?;
        Ok(())
    }
}

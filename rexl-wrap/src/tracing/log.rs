use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{Level, Metadata};
use tracing_appender::rolling::{Builder, RollingFileAppender};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
struct SharedWriter(Arc<Mutex<RollingFileAppender>>);

impl Write for SharedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| std::io::Error::other("mutex poisoned"))?
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0
            .lock()
            .map_err(|_| std::io::Error::other("mutex poisoned"))?
            .flush()
    }
}

pub struct LeveledRollingFileAppender {
    error_appender: SharedWriter,
    warn_appender:  SharedWriter,
    info_appender:  SharedWriter,
}

impl LeveledRollingFileAppender {
    pub fn new<F: Fn() -> Builder>(builder: F, log_dir: &str) -> Self {
        Self {
            error_appender: Self::new_shared_writer(builder(), log_dir, "error"),
            warn_appender:  Self::new_shared_writer(builder(), log_dir, "warn"),
            info_appender:  Self::new_shared_writer(builder(), log_dir, "info"),
        }
    }

    fn new_shared_writer(builder: Builder, log_dir: &str, level: &str) -> SharedWriter {
        SharedWriter(Arc::new(Mutex::new(
            builder.filename_prefix(level).build(log_dir).unwrap(),
        )))
    }
}

impl MakeWriter<'_> for LeveledRollingFileAppender {
    type Writer = Box<dyn Write + Send>;

    fn make_writer(&self) -> Self::Writer {
        Box::new(std::io::stdout())
    }

    fn make_writer_for(&self, meta: &Metadata<'_>) -> Self::Writer {
        match meta.level() {
            &Level::ERROR => Box::new(self.error_appender.clone()),
            &Level::WARN => Box::new(self.warn_appender.clone()),
            _ /* info, debug, trace */ => Box::new(self.info_appender.clone()),
        }
    }
}

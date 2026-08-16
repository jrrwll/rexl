use std::fmt::Display;

const SEPARATOR: &str = " <- ";

pub fn format_error_compact(err: &anyhow::Error) -> String {
    let mut parts = Vec::new();
    parts.push(err.to_string());
    let mut source = err.source();
    while let Some(cause) = source {
        parts.push(cause.to_string());
        source = cause.source();
    }
    parts.reverse();
    parts.join(SEPARATOR)
}

pub trait ResultExt<T, E> {
    #[track_caller]
    fn here(self) -> anyhow::Result<T>
    where E: Into<anyhow::Error>;

    #[track_caller]
    fn here_context<S: Display>(self, context: S) -> anyhow::Result<T>
    where E: Into<anyhow::Error>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn here(self) -> anyhow::Result<T>
    where E: Into<anyhow::Error> {
        let loc = std::panic::Location::caller();
        self.map_err(|e| {
            e.into()
                .context(format!("(at {}:{})", loc.file(), loc.line()))
        })
    }

    #[track_caller]
    fn here_context<S: Display>(self, context: S) -> anyhow::Result<T>
    where E: Into<anyhow::Error> {
        let loc = std::panic::Location::caller();
        self.map_err(|e| {
            e.into()
                .context(format!("{} (at {}:{})", context, loc.file(), loc.line()))
        })
    }
}

pub mod argparse;
pub mod io;
pub mod math;
pub mod mime;
pub mod sort;
pub mod text;
pub mod time;

#[cfg(feature = "serde")]
pub mod api;
#[cfg(feature = "lettre")]
pub mod mail;

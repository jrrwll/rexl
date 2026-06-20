pub use self::rfind::*;
pub use self::size::*;

mod arg_parse;
mod context;
mod i18n;
mod rfind;
mod size;
mod time;
mod util;

#[cfg(test)]
mod rfind_test;

use crate::context::Context;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// walk a file hierarchy
pub fn main() {
    let args = env::args().skip(1).collect();
    let context = Context::new();
    let mut main = Main::new(args, &context);
    main.run(&context);
}

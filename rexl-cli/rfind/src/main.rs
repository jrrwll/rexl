pub use self::arg_convert::*;
pub use self::arg_parse::*;
pub use self::context::*;
pub use self::i18n::*;
pub use self::options::*;
pub use self::rfind::*;

mod arg_convert;
mod arg_parse;
mod context;
mod i18n;
mod options;
mod rfind;

#[cfg(test)]
mod rfind_test;

use crate::arg_parse::parse_args;
use crate::context::Context;
use crate::i18n::load_i18n_config;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// walk a file hierarchy
pub fn main() {
    let args = env::args().skip(1).collect();
    let (usage, message) = unsafe { load_i18n_config() };
    println!("{:?}", &args);
    let context = Context::new(usage, message);
    let mut main = parse_args(&context, args);
    main.run();
}

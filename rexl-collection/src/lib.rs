#![feature(box_syntax)]

pub use self::collection::*;
pub use self::collection_error::*;
pub use self::iterable::*;
pub use self::list::*;
pub use self::queue::*;

pub mod prelude;

mod collection;
mod collection_error;
mod iterable;
mod list;
mod queue;

#![recursion_limit="256"]

extern crate num;

mod obj;
mod parser;
mod convertors;
#[macro_use]
mod macros;
mod value;




pub use self::obj::Obj;
pub use self::parser::{Parser, ParserErr};
pub use self::value::Val;

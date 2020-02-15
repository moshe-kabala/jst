#![recursion_limit="256"]

extern crate num;

mod obj;
mod parser;
#[macro_use]
mod value;


pub use self::obj::Json;
pub use self::parser::{Parser, ParserErr};
pub use self::value::Value;

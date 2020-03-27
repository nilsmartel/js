extern crate gc;
extern crate gc_derive;

mod compile;
pub mod parse;
pub use parse::parse;
mod vm;

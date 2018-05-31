#![feature(range_contains, box_patterns, iterator_repeat_with)]

mod parser;
mod lexer;

pub use parser::{Parser, eval};
#![feature(range_contains, box_patterns, iterator_repeat_with)]

mod interpreter;
mod lexer;
mod parser;

pub mod prelude {
    pub use interpreter::eval;
}

#![feature(box_patterns, iterator_repeat_with)]

macro_rules! error {
    ($($tt:tt)*) => ({
        eprintln!($($tt)*);
        std::process::exit(1);
    });
}

mod interpreter;
mod lexer;
mod parser;
mod source_stream;

pub mod prelude {
    pub use interpreter::eval;
}

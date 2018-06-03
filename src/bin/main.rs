extern crate tachyon;

use std::io::Write;
use tachyon::prelude::*;

fn main() {
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();

        let mut input_buf = String::new();
        std::io::stdin().read_line(&mut input_buf).unwrap();

        if let Some(res) = eval(&input_buf) {
            println!("{}", res);
        }
    }
}

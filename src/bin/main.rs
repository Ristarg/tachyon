extern crate tachyon;

use std::io::Write;
use tachyon::prelude::*;

fn main() {
    let mut input_buf = String::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input_buf).unwrap();

        let res = eval(&input_buf);

        println!("{}", res);
        input_buf.clear();
    }
}

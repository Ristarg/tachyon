extern crate tachyon;

use tachyon::prelude::*;

fn main() {
    loop {
        print_prompt();

        // read
        let mut input_buf = String::new();
        std::io::stdin().read_line(&mut input_buf).unwrap();

        // eval
        let res = eval(&input_buf);

        // print
        println!("{}", res);
    } // loop!
}

fn print_prompt() {
    use std::io::Write;

    print!(">>> ");
    std::io::stdout().flush().unwrap();
}

use std::io::{self, Write};
use std::env;
use std::fs;

use rlox::vm::VM;

fn main() {
    let args: Vec<String>  = env::args().collect();    
    let mut vm: VM = Default::default();

    if args.len() == 1 {
        loop {
            print!("> ");
            io::stdout().flush().expect("Flush Failure.");
            let mut line = String::new();
            let stdin = io::stdin();
            match stdin.read_line(&mut line) {
                Ok(_n) => {
                    vm.interpret(line, &mut io::stdout());
                }
                Err(error) => println!("error: {}", error),
            }
        }
    } else {
        let filename = &args[1];
        let contents = fs::read_to_string(filename).expect("Cannot read file.");

        vm.interpret(contents, &mut io::stdout());
    }
}

use std::io::{self, Write};
use std::process;
use std::env;
use std::fs;

pub mod scanner;
pub mod chunk;
pub mod vm;
pub mod value;

use scanner::Scanner;
use chunk::{Chunk, OpCode};
use vm::*;

fn main() {
    let args: Vec<String>  = env::args().collect();    
    let mut chunk: Chunk = Default::default();

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::Constant, 1);
    chunk.write_chunk(OpCode::OpArg(constant), 1);

    let constant = chunk.add_constant(1.4);
    chunk.write_chunk(OpCode::Constant, 1);
    chunk.write_chunk(OpCode::OpArg(constant), 1);

    chunk.write_chunk(OpCode::Add, 1);

    let constant = chunk.add_constant(2.0);
    chunk.write_chunk(OpCode::Constant, 1);
    chunk.write_chunk(OpCode::OpArg(constant), 1);

    chunk.write_chunk(OpCode::Divide, 1);

    chunk.write_chunk(OpCode::Return, 2);

    let vm: VM = Default::default();
    vm.interpret(chunk);

    if args.len() == 1 {
        loop {
            print!("> ");
            io::stdout().flush().expect("Flush Failure.");
            let mut line = String::new();
            let stdin = io::stdin();
            match stdin.read_line(&mut line) {
                Ok(_n) => {
                    let mut scanner = Scanner::new(line);
                    if let Err(e) = scanner.run() {
                        println!("Scanner error. {}", e);
                        process::exit(1);
                    }
                }
                Err(error) => println!("error: {}", error),
            }
        }
    } else {
        let filename = &args[1];
        let contents = fs::read_to_string(filename).expect("Cannot read file.");
        let mut scanner = Scanner::new(contents);
        if let Err(e) = scanner.run() {
            println!("Scanner error. {}", e);
            process::exit(1);
        }
    }
}

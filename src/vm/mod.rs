use std::io;

use crate::chunk::*;
use crate::value::*;
use crate::compiler::*;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>
}

impl VM {
    pub fn interpret(&mut self, source: String, out: &mut io::Write) -> InterpretResult {
        let mut compiler: Compiler = Compiler::new(source);
        
        let chunk = match compiler.compile() {
            Ok(chunk) => chunk,
            Err(_e) => return InterpretResult::CompileError
        };

        self.ip = 0;
        self.chunk = chunk;
        
        self.run(out);
        InterpretResult::Ok
    }

    fn advance(&mut self) -> OpCode {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&self, arg: OpCode) -> Value {
        if let OpCode::OpArg(a) = arg {
            self.chunk.constants[a]
        } else {
            panic!("Expect an argument")
        }
    }

    fn binary_op(&mut self, op: char) {
       if let Some(opnd1) = self.stack.pop() {
            if let Some(opnd2) = self.stack.pop() {
                let result = match op {
                    '+' => opnd2 + opnd1,
                    '-' => opnd2 - opnd1,
                    '*' => opnd2 * opnd1,
                    '/' => opnd2 / opnd1,
                     _  => { 0.0 }
                };
                self.stack.push(result);
            }
        }
    }

    fn run(&mut self, out: &mut io::Write) -> InterpretResult {

        loop {
            debug::disassemble_instruction(&self.chunk, self.ip);
            println!("{:?}", self.stack);

            let instruction = self.advance();

            match instruction {
                OpCode::Return => {
                    write!(out, "{:?}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                },

                OpCode::Constant => {
                    let arg = self.advance();
                    let value = self.read_constant(arg);
                    self.stack.push(value);
                },

                OpCode::Negate => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(-value);
                    }
                },

                OpCode::Add => {
                    self.binary_op('+');                    
                },

                OpCode::Subtract => {
                    self.binary_op('-');                    
                },

                OpCode::Multiply => {
                    self.binary_op('*');                    
                },

                OpCode::Divide => {
                    self.binary_op('/');                    
                }


                _ => {}
            }
        }
    }
}

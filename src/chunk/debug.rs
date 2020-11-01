use super::*;
use crate::value;

pub fn disassemble_chunk(chunk: &Chunk) {
    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 &&
       chunk.lines[offset] == chunk.lines[offset-1]{
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = &chunk.code[offset];

    match instruction {
        OpCode::Constant => return constant_instruction("CONSTANT", chunk, offset),
        OpCode::Negate => return simple_instruction("NEGATE", offset),
        OpCode::Return => return simple_instruction("RETURN", offset),
        OpCode::Add => return simple_instruction("ADD", offset),
        OpCode::Subtract => return simple_instruction("SUBTRACT", offset),
        OpCode::Multiply => return simple_instruction("MULTIPLY", offset),
        OpCode::Divide => return simple_instruction("DIVIDE", offset),
        _ => {}
    }
    offset
}

fn simple_instruction(op_name: &str, offset: usize) -> usize {
    println!("{}", op_name);
    offset + 1
}

fn constant_instruction(op_name: &str, chunk: &Chunk, offset: usize) -> usize {
    if let OpCode::OpArg(index) = &chunk.code[offset + 1] {
        print!("{:16} {:4} ", op_name, index);
        value::print_value(chunk.constants[*index]);
        println!();
    }
    offset + 2
}

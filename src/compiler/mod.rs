use std::error::Error;

use crate::chunk::*;
use crate::scanner::*;
use crate::value::*;

#[derive(PartialOrd, PartialEq)]
#[derive(Copy, Clone)]
enum Precedence {
    None = 0,
    Assignment = 1,
    Or = 2,
    And = 3,
    Equality = 4,
    Comparision = 5,
    Term = 6,
    Factor = 7,
    Unary = 8,
    Call = 9,
    Primary = 10
}

impl Default for Precedence {
    fn default() -> Precedence {
        Precedence::None
    }
}

impl Precedence {
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparision,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            _ => Precedence::Primary,
        }
    }
}

type ParseFn =  Option<Box<Fn(&mut Compiler)>>;

#[derive(Default)]
pub struct ParseRule(
    ParseFn,
    ParseFn,
    Precedence
);

#[derive(Default)]
pub struct Compiler {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner,
    chunk: Chunk,
    current_precedence: Precedence
}

pub struct ParseRules {
    rules: [ParseRule; 32]
}

impl Default for ParseRules {
    fn default() -> ParseRules {
        let mut rules:[ParseRule; 32] = Default::default();
        rules[TokenType::Plus as usize] = ParseRule(None, Some(Box::new(binary)), Precedence::Term);
        rules[TokenType::Minus as usize] = ParseRule(Some(Box::new(unary)), Some(Box::new(binary)), Precedence::Term);
        rules[TokenType::Star as usize] = ParseRule(None, Some(Box::new(binary)), Precedence::Factor);
        rules[TokenType::Slash as usize] = ParseRule(None, Some(Box::new(binary)), Precedence::Factor);
        rules[TokenType::Number as usize] = ParseRule(Some(Box::new(number)), None, Precedence::Term);
        rules[TokenType::LeftParen as usize] = ParseRule(Some(Box::new(grouping)), None, Precedence::Term);

        ParseRules {
            rules
        }
    }
}

impl Compiler {
    pub fn new (source: String) -> Self {
        Self {
            scanner: Scanner::new(source),
            ..Default::default()
        }
    }

    fn error_at(&self, token: &Token, message: String) {
        if self.panic_mode {
            return
        }

        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else {
            eprint!(" at {}", token.lexeme);
        }

        eprintln!(": {}", message);
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(&self.current, message);
        self.had_error = true;
        self.panic_mode = true;
    }

    fn error(&mut self, message: String) {
        self.error_at(&self.previous, message);
        self.had_error = true;
        self.panic_mode = true;
    }

    fn advance(&mut self) {
        loop {
            self.previous = std::mem::take(&mut self.current);
            let token = self.scanner.next();
            
            match token {
                Some(token) => {
                    self.current = token;
                    if self.current.token_type != TokenType::Error {
                        break;
                    }
                    self.error_at_current(String::from(&self.current.lexeme));
                },
                None => break
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit_bytes(OpCode::Constant, OpCode::OpArg(index));
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let index = self.chunk.add_constant(value);
        if index > u8::MAX as usize {
            self.error(String::from("Too many constants in one chunk."));
            return 0;
        }
        index
    }

    pub fn compile(&mut self) -> Result<Chunk, Box<dyn Error>> {
        self.advance();
        expression(self);
        self.end_compiler();
        Ok(std::mem::take(&mut self.chunk))
    }
}

fn parse_precedence(compiler: &mut Compiler, precedence: Precedence) {
    let parse_rules: ParseRules = Default::default();
    let rules = parse_rules.rules;
    compiler.advance();

    let prefix = rules[compiler.previous.token_type as usize].0.as_ref().unwrap();
    prefix(compiler);

    while precedence <= rules[compiler.current.token_type as usize].2 {
        compiler.advance();
        let rule = &rules[compiler.previous.token_type as usize];
        // print!("{:?}", compiler.previous.token_type);
        let infix = rule.1.as_ref().unwrap();
        compiler.current_precedence = rule.2;
        infix(compiler);
    }
}

fn number(compiler: &mut Compiler) {
    let value = compiler.previous.lexeme.parse::<Value>().unwrap();
    compiler.emit_constant(value);
}

fn expression(compiler: &mut Compiler) {
    parse_precedence(compiler, Precedence::Assignment);
}

fn unary(compiler: &mut Compiler) {
    let operator = compiler.previous.token_type;
    
    parse_precedence(compiler, Precedence::Unary);

    match operator {
        TokenType::Minus => compiler.emit_byte(OpCode::Negate),
        _ => {}
    }
}

fn binary(compiler: &mut Compiler) {
    let operator = compiler.previous.token_type;

    parse_precedence(compiler, Precedence::from_u8(compiler.current_precedence as u8));
    
    match operator {
        TokenType::Plus => compiler.emit_byte(OpCode::Add),
        TokenType::Minus => compiler.emit_byte(OpCode::Subtract),
        TokenType::Star => compiler.emit_byte(OpCode::Multiply),
        TokenType::Slash => compiler.emit_byte(OpCode::Divide),
        _ => {}
    }
}


fn grouping(compiler: &mut Compiler) {
    expression(compiler);
    compiler.consume(TokenType::RightParen, String::from("Expect ')' after expression."));
}


use std::error::Error;

use crate::chunk::*;
use crate::scanner::*;

#[derive(Default)]
pub struct Compiler {

}

#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token
}


impl Compiler {
    pub fn compile(self, source: String) -> Result<Chunk, Box<dyn Error>>{
        let chunk: Chunk = Default::default();

        self.advance();
        self.expression();
        self.consume(TokenType::EOF);
        Ok(chunk)
    }
}

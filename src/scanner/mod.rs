use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen,
  LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus,
  Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False,
  For, Fun, If, Nil, Or,
  Print, Return, Super, This,
  True, Var, While,

  Error
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug)]
pub struct Scanner {
    contents: String,
    start: usize,
    current: usize,
    line: usize,
}


impl Token {
    fn new(token_type: TokenType, lexeme: String,
           line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line
        }
    }
}

impl Scanner {
    pub fn new(contents: String) -> Self {
        Self { 
            contents,
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        for tokens in self {
            println!("{:?}", tokens);
        }
        Ok(())
    }

    fn make_token(&self, token: TokenType) -> Token {
        Token::new(token,
                self.contents[self.start..self.current].to_string(), 
                self.line)
    }

    fn make_error_token(&self, message: String) -> Token {
        Token::new(TokenType::Error,
                message, 
                self.line)
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_end() { return };
            let c = self.peek();
            match c {
                ' ' | '\t' | '\r' => { self.advance(); },
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.can_peek_next() && self.peek_next() == '/' {
                        while !self.is_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    }
                    return;
                }
                _ => return
            }
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_end() { return false };
        if self.peek() != expected { return false }

        self.current += 1;
        true
    }

    fn is_end(&self) -> bool {
        self.current == self.contents.len()
    }

    fn can_peek_next(&self) -> bool {
        !self.is_end() && self.current != self.contents.len() - 1
    }


    fn peek(&self) -> char {
        self.contents.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        self.contents.as_bytes()[self.current + 1] as char
    }

    fn string(&mut self) -> Token {
        while !self.is_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() { return self.make_error_token(String::from("Unterminated String")) }
        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while !self.is_end() && is_digit(self.peek()) {
            self.advance();
        }

        if !self.is_end() && self.peek() == '.' {
            self.advance();

            while !self.is_end() && is_digit(self.peek()) {
                self.advance();
            }

        }
        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while !self.is_end() && is_alphanumeric(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        let c0 = self.contents.as_bytes()[self.start] as char;

        match c0 {
            'a' => self.check_keyword(self.start + 1, 2, "ar", TokenType::And),
            'c' => self.check_keyword(self.start + 1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(self.start + 1, 3, "lse", TokenType::Else),
            'i' => self.check_keyword(self.start + 1, 1, "f", TokenType::If),
            'n' => self.check_keyword(self.start + 1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(self.start + 1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(self.start + 1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(self.start + 1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(self.start + 1, 4, "uper", TokenType::Super),
            'v' => self.check_keyword(self.start + 1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(self.start + 1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start > 1 {
                    let c1 = self.contents.as_bytes()[self.start + 1] as char;

                    match c1 {
                        'a' => self.check_keyword(self.start + 2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(self.start + 2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(self.start + 2, 1, "n", TokenType::Fun),
                         _  => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    let c1 = self.contents.as_bytes()[self.start + 1] as char;

                    match c1 {
                        'h' => self.check_keyword(self.start + 2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(self.start + 2, 2, "ue", TokenType::True),
                         _  => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
             _  => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, length: usize,
                     rest: &str, token_type: TokenType)
                     -> TokenType {
        if self.current - start == length &&
           self.contents[start..start + length].eq(rest) {
            return token_type
        }
        TokenType::Identifier
    }

}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'A' && c <= 'Z') ||
    (c >= 'a' && c <= 'z') ||
    c == '_'
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_end() { return None }

        let c = self.advance();

        match c {
            '(' => return Some(self.make_token(TokenType::LeftParen)),
            ')' => return Some(self.make_token(TokenType::RightParen)),
            '{' => return Some(self.make_token(TokenType::LeftBrace)),
            '}' => return Some(self.make_token(TokenType::RightBrace)),
            ';' => return Some(self.make_token(TokenType::Semicolon)),
            ',' => return Some(self.make_token(TokenType::Comma)),
            '.' => return Some(self.make_token(TokenType::Dot)),
            '-' => return Some(self.make_token(TokenType::Minus)),
            '+' => return Some(self.make_token(TokenType::Plus)),
            '/' => return Some(self.make_token(TokenType::Slash)),
            '*' => return Some(self.make_token(TokenType::Star)),

            '!' => {
                    let token = if self.matches('=') {
                        TokenType::BangEqual
                    } else { 
                        TokenType::Bang 
                    };
                    return Some(self.make_token(token))
            },
            '=' => {
                    let token = if self.matches('=') {
                        TokenType::EqualEqual
                    } else { 
                        TokenType::Equal
                    };
                    return Some(self.make_token(token))
            },
            '<' => {
                    let token = if self.matches('=') {
                        TokenType::LessEqual
                    } else { 
                        TokenType::Less 
                    };
                    return Some(self.make_token(token))
            },
            '>' => {
                    let token = if self.matches('=') {
                        TokenType::GreaterEqual
                    } else { 
                        TokenType::Greater
                    };
                    return Some(self.make_token(token))
            },

            '"' => return Some(self.string()),
            '0'..='9' => return Some(self.number()),
            'a'..='z' | 'A'..='Z' => return Some(self.identifier()),
            _ => {}
        }

        Some(self.make_error_token(String::from("unexpected character")))
    }
}

#[cfg(test)]
mod test;

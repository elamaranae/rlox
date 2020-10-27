use super::{Scanner, TokenType};

#[test]
fn expressions() {
    let mut scanner = Scanner::new(String::from("var a = (x-y)*(y/x)"));
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Var);  
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Equal);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Minus);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Star);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Slash);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
}

#[test]
fn identifier() {
    let mut scanner = Scanner::new(String::from("var dsf23 = 3;"));
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Var);  
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Equal);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Number);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Semicolon);
}

#[test]
fn keyword_class() {
    let mut scanner = Scanner::new(String::from("
                                                    class Maya {
                                                        fun hey() {
                                                            return 5;
                                                        }
                                                    }
                                                "));
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Class);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Fun);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Return);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Number);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Semicolon);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightBrace);
}

#[test]
fn keyword_for() {
    let mut scanner = Scanner::new(String::from("
                                                    for (var a=1) { 
                                                        if (f == b) return a;
                                                    }
                                                f"));
    assert_eq!(scanner.next().unwrap().token_type, TokenType::For);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Var);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Equal);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Number);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::If);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::EqualEqual);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Return);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Semicolon);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
}

#[test]
fn keyword_this() {
    let mut scanner = Scanner::new(String::from("
                                                    for (var a==true) { 
                                                        if (this.f == false) return a;
                                                    }
                                                f"));
    assert_eq!(scanner.next().unwrap().token_type, TokenType::For);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Var);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::EqualEqual);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::True);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::If);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::LeftParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::This);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Dot);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::EqualEqual);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::False);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightParen);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Return);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Semicolon);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::RightBrace);
    assert_eq!(scanner.next().unwrap().token_type, TokenType::Identifier);
}

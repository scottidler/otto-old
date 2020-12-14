use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn tokenize(&self, args: Vec<String>) -> Vec<Token> {
        let mut tokens = vec![];
        for arg in args {
            if arg.starts_with("--") {
                tokens.push(Token::LongId(arg[2..].to_string()));
            }
            else if arg.starts_with("-") {
                tokens.push(Token::ShortId(arg[1..].to_string()));
            }
            else {
                tokens.push(Token::ValueId(arg));
            }
        }
        tokens
    }
}

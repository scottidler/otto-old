use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TSK(String),
    SHT(String),
    LNG(String),
    CHO(String),
    POS(String),
    VAL(String),
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::TSK(s) => write!(formatter, "TSK[{}]",s),
            Token::SHT(s) => write!(formatter, "SHT[{}]",s),
            Token::LNG(s) => write!(formatter, "LNG[{}]",s),
            Token::CHO(s) => write!(formatter, "CHO[{}]",s),
            Token::POS(s) => write!(formatter, "POS[{}]",s),
            Token::VAL(s) => write!(formatter, "VAL[{}]",s),
            Token::EOF => write!(formatter, "EOF"),
        }
    }
}

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BLT(String),
    KWD(String),
    ARG(String),
    VAL(String),
    KVP(String),
    REM(Vec<String>),
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::BLT(s) => write!(fmtr, "Token::BLT[{}]", s),
            Token::KWD(s) => write!(fmtr, "Token::KWD[{}]", s),
            Token::ARG(s) => write!(fmtr, "Token::ARG[{}]", s),
            Token::VAL(s) => write!(fmtr, "Token::VAL[{}]", s),
            Token::KVP(s) => write!(fmtr, "Token::KVP[{}]", s),
            Token::REM(vs) => write!(fmtr, "Token::REM[{}]", vs.join(" ")),
            Token::EOF => write!(fmtr, "Token::EOF"),
        }
    }
}
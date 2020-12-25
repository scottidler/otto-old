use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BLT(String),
    KWD(String),
    SHT(String),
    LNG(String),
    VAL(String),
    REM(Vec<String>),
}

impl fmt::Display for Token {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::BLT(s) => write!(fmtr, "BLT[{}]", s),
            Token::KWD(s) => write!(fmtr, "KWD[{}]", s),
            Token::SHT(s) => write!(fmtr, "SHT[{}]", s),
            Token::LNG(s) => write!(fmtr, "LNG[{}]", s),
            Token::VAL(s) => write!(fmtr, "VAL[{}]", s),
            Token::REM(vs) => write!(fmtr, "REM[{}]", vs.join(" ")),
        }
    }
}

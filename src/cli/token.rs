#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ValueId(String),
    ShortId(String),
    LongId(String),
}

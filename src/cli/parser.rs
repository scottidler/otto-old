use super::token::Token;

use crate::cfg::spec::Otto;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub otto: Otto,
}

impl Parser {
    pub fn new(otto: Otto) -> Self {
        Self {
            otto: otto,
        }
    }
}

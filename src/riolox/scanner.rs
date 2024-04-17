use super::token::Token;

pub(crate) struct Scanner {}

impl Scanner {
    pub fn new(_: String) -> Self {
        Scanner {}
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}

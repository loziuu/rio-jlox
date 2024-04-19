use super::{
    error,
    token::{Token, TokenType},
};

pub(crate) struct Scanner {
    // TODO: Use char0
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while self.has_more() {
            self.start = self.current;
            self.scan_token(); // This will probably need to be aligned...
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), "", self.line));
        self.tokens.as_ref()
    }

    fn has_more(&self) -> bool {
        self.current < self.source.len()
    }

    fn scan_token(&mut self) {
        let advanced = self.advance();

        if let Some(c) = advanced {
            match c {
                '(' => {
                    self.push_token(TokenType::LeftParen);
                }
                ')' => {
                    self.push_token(TokenType::RightParen);
                }
                '{' => {
                    self.push_token(TokenType::LeftBrace);
                }
                '}' => {
                    self.push_token(TokenType::RightBrace);
                }
                ',' => {
                    self.push_token(TokenType::Comma);
                }
                '.' => {
                    self.push_token(TokenType::Dot);
                }
                '-' => {
                    self.push_token(TokenType::Minus);
                }
                '+' => {
                    self.push_token(TokenType::Plus);
                }
                ';' => {
                    self.push_token(TokenType::Semicolon);
                }
                '*' => {
                    self.push_token(TokenType::Star);
                }
                '!' => {
                    let t = self.on_next('=', TokenType::BangEqual, TokenType::Bang);
                    self.push_token(t);
                }
                '=' => {
                    let t = self.on_next('=', TokenType::EqualEqual, TokenType::Equal);
                    self.push_token(t);
                }
                '<' => {
                    let t = self.on_next('=', TokenType::LessEqual, TokenType::Less);
                    self.push_token(t);
                }
                '>' => {
                    let t = self.on_next('=', TokenType::GreaterEqual, TokenType::Greater);
                    self.push_token(t);
                }
                '/' => {
                    if self.is_next('/') {
                        self.current += 1;

                        let mut next = self.peek();
                        while next.is_some() && next != Some('\n') {
                            self.advance();
                            next = self.peek();
                        }
                    } else {
                        self.push_token(TokenType::Slash);
                    }
                }
                '\n' => {
                    self.line += 1;
                }
                ' ' | '\r' | '\t' => {}
                '"' => self.string(),
                _ => error(self.line, "Unexpected character."),
            }
        }
    }

    // TODO: Optimize this
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn push_token(&mut self, t: TokenType) {
        // TODO: What if we assume that it's only ASCII?
        let text = self.substring(self.start, self.current);
        self.tokens.push(Token::new(t, text, "", self.line));
    }

    fn push_token_with_value(&mut self, t: TokenType, value: String) {
        self.tokens.push(Token::new(t, value, "", self.line));
    }

    fn substring(&self, start: usize, end: usize) -> String {
        self.source.chars().skip(start).take(end - start).collect()
    }

    fn peek(&self) -> Option<char> {
        if !self.has_more() {
            return None;
        }
        self.char_at(self.current)
    }

    fn char_at(&self, n: usize) -> Option<char> {
        if n >= self.source.len() {
            return None;
        }
        self.source.chars().nth(n)
    }

    // TODO: Change to match. Return bool?
    fn on_next(&mut self, expected: char, on_true: TokenType, on_false: TokenType) -> TokenType {
        if self.is_next(expected) {
            self.current += 1;
            on_true
        } else {
            on_false
        }
    }

    fn is_next(&self, expected: char) -> bool {
        if !self.has_more() {
            return false;
        }

        if let Some(ch) = self.char_at(self.current) {
            ch == expected
        } else {
            false
        }
    }

    fn string(&mut self) {
        let mut peek = self.peek();
        while peek.is_some() && peek != Some('"') {
            if peek == Some('\n') {
                self.line += 1;
            }
            self.advance();
            peek = self.peek();
        }

        if !self.has_more() {
            error(self.line, "Unterminated string")
        }

        self.advance();

        let value = self.substring(self.start + 1, self.current - 1);
        self.push_token_with_value(TokenType::String, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{riolox::token::TokenType, *};

    #[test]
    fn parenthesis() {
        let source = "()".to_owned();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].token_type(), &TokenType::LeftParen);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].token_type(), &TokenType::RightParen);
        assert_eq!(tokens[2].line(), 1);
        assert_eq!(tokens[2].token_type(), &TokenType::Eof);
    }

    #[test]
    fn braces() {
        let source = "{}".to_owned();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].token_type(), &TokenType::LeftBrace);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].token_type(), &TokenType::RightBrace);
        assert_eq!(tokens[2].line(), 1);
        assert_eq!(tokens[2].token_type(), &TokenType::Eof);
    }

    #[test]
    fn string_in_braces() {
        let source = "{ \"hello\" }".to_owned();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].token_type(), &TokenType::LeftBrace);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].token_type(), &TokenType::String);
        assert_eq!(tokens[1].lexeme(), "hello");
        assert_eq!(tokens[2].line(), 1);
        assert_eq!(tokens[2].token_type(), &TokenType::RightBrace);
        assert_eq!(tokens[3].line(), 1);
        assert_eq!(tokens[3].token_type(), &TokenType::Eof);
    }

    #[test]
    fn operators() {
        let source = "!*+-/=<> <= == // operators".to_owned();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        dbg!(&tokens);

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token_type(), &TokenType::Bang);
        assert_eq!(tokens[1].token_type(), &TokenType::Star);
        assert_eq!(tokens[2].token_type(), &TokenType::Plus);
        assert_eq!(tokens[3].token_type(), &TokenType::Minus);
        assert_eq!(tokens[4].token_type(), &TokenType::Slash);
        assert_eq!(tokens[5].token_type(), &TokenType::Equal);
        assert_eq!(tokens[6].token_type(), &TokenType::Less);
        assert_eq!(tokens[7].token_type(), &TokenType::Greater);
        assert_eq!(tokens[8].token_type(), &TokenType::LessEqual);
        assert_eq!(tokens[9].token_type(), &TokenType::EqualEqual);
        assert_eq!(tokens[10].token_type(), &TokenType::Eof);
    }
}

use crate::{
    error::FeroxError,
    token::{Token, TokenType},
};

#[derive(Default)]
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    line_number: usize,
    current: usize,
    pub errors: Vec<FeroxError>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line_number: 0,
            current: 0,
            start: 0,
            errors: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, FeroxError> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token(&mut tokens);
        }

        tokens.push(Token::new(TokenType::Eof, String::new(), self.line_number));

        Ok(tokens)
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) {
        if let Some(c) = self.advance() {
            match c {
                // Always single character
                '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                    if let Ok(token_type) = TokenType::try_from(c) {
                        self.add_token(tokens, token_type);
                    } else {
                        self.errors.push(FeroxError::SyntaxError {
                            error_description: "Unexpected character".to_owned(),
                            line_number: self.line_number,
                        });
                    }
                }
                // Always single or double character
                '!' | '=' | '<' | '>' => {
                    if let Some(token_type) = self.single_or_double_character_token_type(c) {
                        self.add_token(tokens, token_type);
                    } else {
                        self.errors.push(FeroxError::SyntaxError {
                            error_description: "Unexpected character".to_owned(),
                            line_number: self.line_number,
                        });
                    }
                }
                // Start of comment or slash
                '/' => {
                    if self.match_current('/') {
                        while self.peek().is_some() && !self.is_at_end() {
                            _ = self.advance();
                        }
                    } else {
                        self.add_token(tokens, TokenType::Slash);
                    }
                }
                // Ignore
                ' ' | '\r' | '\t' | '\n' => {
                    if c == '\n' {
                        self.line_number += 1;
                    }
                }
                // Start of string literal
                '"' => {
                    self.handle_string_literal(tokens);
                }
                _ => {
                    self.errors.push(FeroxError::SyntaxError {
                        error_description: "Unexpected character".to_owned(),
                        line_number: self.line_number,
                    });
                }
            }
        }
    }

    fn add_token(&self, tokens: &mut Vec<Token>, token_type: TokenType) {
        let token = Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            self.line_number,
        );

        tokens.push(token);
    }

    fn add_token_literal(&self, tokens: &mut Vec<Token>, token_type: TokenType) {
        let token = Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            self.line_number,
        );

        tokens.push(token);
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if let Some(c) = self.source.get(self.current) && *c != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> Option<char> {
        let current_char = self.source.get(self.current);

        self.current += 1;

        current_char.copied()
    }

    fn peek(&self) -> Option<char> {
        let current_char = self.source.get(self.current);

        current_char.copied()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn should_ignore(&self, c: char) -> bool {
        matches!(c, ' ' | '\r' | '\t' | '\n')
    }

    fn handle_string_literal(&mut self, tokens: &mut Vec<Token>) {
        while let Some(c) = self.peek() && c != '"' && !self.is_at_end() {
            if c == '\n' {
                self.line_number += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(FeroxError::SyntaxError {
                error_description: "Unterminated string".to_owned(),
                line_number: self.line_number,
            })
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token(tokens, TokenType::String { value });
    }

    fn single_or_double_character_token_type(&mut self, c: char) -> Option<TokenType> {
        match c {
            '!' => Some(if self.match_current('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => Some(if self.match_current('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => Some(if self.match_current('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => Some(if self.match_current('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            _ => None,
        }
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line_number,
        }
    }
}

pub enum TokenType {
    // Single character tokens
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier { identifier: String },
    String { value: String },
    Number { value: f64 },
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    FOr,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Other
    Eof,
}

impl TryFrom<char> for TokenType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(TokenType::LeftParentheses),
            ')' => Ok(TokenType::RightParentheses),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::SemiColon),
            '*' => Ok(TokenType::Star),
            _ => Err(()),
        }
    }
}
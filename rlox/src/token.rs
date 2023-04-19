#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: i32) -> Token {
        Token { typ, lexeme, line }
    }

    pub fn to_string(&self) -> String {
        use TokenType::*;
        match &self.typ {
            Identifier(literal) => {
                format!("<TokenType::{:?}, {}, {}>", self.typ, self.lexeme, literal)
            }
            String(literal) => format!("<TokenType::{:?}, {}, {}>", self.typ, self.lexeme, literal),
            Number(literal) => format!("<TokenType::{:?}, {}, {}>", self.typ, self.lexeme, literal),
            _ => format!("<TokenType::{:?}, {}>", self.typ, self.lexeme),
        }
    }
}

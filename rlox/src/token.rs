use std::fmt::Display;

use phf::phf_map;

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

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

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
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        match &self.typ {
            Identifier(literal) => {
                write!(
                    f,
                    "<TokenType::{:?}, {}, {}>",
                    self.typ, self.lexeme, literal
                )
            }
            String(literal) => write!(
                f,
                "<TokenType::{:?}, {}, {}>",
                self.typ, self.lexeme, literal
            ),
            Number(literal) => write!(
                f,
                "<TokenType::{:?}, {}, {}>",
                self.typ, self.lexeme, literal
            ),
            _ => write!(f, "<TokenType::{:?}, {}>", self.typ, self.lexeme),
        }
    }
}

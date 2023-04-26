use std::iter::FromIterator;

use crate::error_reporter::ErrorReporter;
use crate::token::{Token, TokenType, KEYWORDS};

pub struct Scanner<'a> {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,

    error_reporter: &'a mut ErrorReporter,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, error_reporter: &'a mut ErrorReporter) -> Scanner {
        let source = source.chars().collect::<Vec<_>>();

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), self.line));
        self.tokens.to_owned()
    }

    fn scan_token(&mut self) {
        use TokenType::*;

        let c = self.advance();
        match *c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(SemiColon),
            '*' => self.add_token(Star),
            '!' => {
                if self.is_same('=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }
            '=' => {
                if self.is_same('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            '>' => {
                if self.is_same('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '<' => {
                if self.is_same('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }
            '/' => {
                if self.is_same('/') {
                    self.scan_inline_comment();
                } else if self.is_same('*') {
                    self.scan_block_comment();
                } else {
                    self.add_token(Slash);
                }
            }
            '"' => self.scan_string(),
            '_' | 'a'..='z' | 'A'..='Z' => self.scan_identifier(),
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            c => {
                if self.is_alpha(c) {
                    self.scan_identifier();
                } else if self.is_numeric(c) {
                    self.scan_number();
                } else {
                    self.error_reporter.error(
                        self.line,
                        format!("Unrecognized character: {}", c).as_str(),
                        exitcode::DATAERR,
                    );
                }
            }
        }
    }

    fn add_token(&mut self, typ: TokenType) {
        let lexeme = self.get_lexeme();
        let token = Token::new(typ, lexeme, self.line);

        self.tokens.push(token);
    }

    fn scan_inline_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn scan_block_comment(&mut self) {
        // Consume "*" in the opening "/*" of the block comment.
        self.advance();

        while !(self.is_same('*') && self.peek_next() == '/' || self.is_at_end()) {
            if self.is_same('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_reporter
                .error(self.line, "Unterminated block comment.", exitcode::DATAERR);
            return;
        }

        // Consume the closing "*/" of the block comment.
        self.advance();
        self.advance();
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_reporter
                .error(self.line, "Unterminated string.", exitcode::DATAERR);
            return;
        }

        // The closing ".
        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        self.add_token(TokenType::String(value));
    }

    fn scan_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        if let Some(reserved_type) = KEYWORDS.get(value.as_str()) {
            self.add_token(reserved_type.clone());
        } else {
            self.add_token(TokenType::Identifier(value));
        }
    }

    fn scan_number(&mut self) {
        while self.is_numeric(self.peek()) {
            self.advance();
        }

        // Consume "." and proceed consuming the real part of the number.
        if self.is_same('.') && self.is_numeric(self.peek_next()) {
            self.advance();

            while self.is_numeric(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f32>()
            .unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> &char {
        let c = &self.source[self.current];
        self.current += 1;

        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_same(&self, c: char) -> bool {
        self.peek() == c
    }

    fn get_lexeme(&self) -> String {
        String::from_iter(&self.source[self.start..self.current])
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_numeric(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_numeric(c)
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenType::*, *};

    #[test]
    fn scan_hello() {
        let source = "print \"Hello, world!\";".to_string();
        let mut error_reporter = ErrorReporter::new();

        let mut scanner = Scanner::new(source, &mut error_reporter);
        let tokens = scanner.scan_tokens();
        let correct = vec![
            Token {
                typ: Print,
                lexeme: "print".to_string(),
                line: 1,
            },
            Token {
                typ: String("Hello, world!".to_string()),
                lexeme: "\"Hello, world!\"".to_string(),
                line: 1,
            },
            Token {
                typ: SemiColon,
                lexeme: ";".to_string(),
                line: 1,
            },
            Token {
                typ: EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        tokens
            .into_iter()
            .zip(correct)
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn scan_fib() {
        let source = "fun fib(n) {
            if (n == 1 or n == 0) {
                return n;
            }
        
            return fib(n-1) + fib(n-2);
        }"
        .to_string();
        let mut error_reporter = ErrorReporter::new();

        let mut scanner = Scanner::new(source, &mut error_reporter);
        let tokens = scanner.scan_tokens();
        let correct = vec![
            Token {
                typ: Fun,
                lexeme: "fun".to_string(),
                line: 1,
            },
            Token {
                typ: Identifier("fib".to_string()),
                lexeme: "fib".to_string(),
                line: 1,
            },
            Token {
                typ: LeftParen,
                lexeme: "(".to_string(),
                line: 1,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 1,
            },
            Token {
                typ: RightParen,
                lexeme: ")".to_string(),
                line: 1,
            },
            Token {
                typ: LeftBrace,
                lexeme: "{".to_string(),
                line: 1,
            },
            Token {
                typ: If,
                lexeme: "if".to_string(),
                line: 2,
            },
            Token {
                typ: LeftParen,
                lexeme: "(".to_string(),
                line: 2,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 2,
            },
            Token {
                typ: EqualEqual,
                lexeme: "=".to_string(),
                line: 2,
            },
            Token {
                typ: Equal,
                lexeme: "=".to_string(),
                line: 2,
            },
            Token {
                typ: Number(1.0),
                lexeme: "1".to_string(),
                line: 2,
            },
            Token {
                typ: Or,
                lexeme: "or".to_string(),
                line: 2,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 2,
            },
            Token {
                typ: EqualEqual,
                lexeme: "=".to_string(),
                line: 2,
            },
            Token {
                typ: Equal,
                lexeme: "=".to_string(),
                line: 2,
            },
            Token {
                typ: Number(0.0),
                lexeme: "0".to_string(),
                line: 2,
            },
            Token {
                typ: RightParen,
                lexeme: ")".to_string(),
                line: 2,
            },
            Token {
                typ: LeftBrace,
                lexeme: "{".to_string(),
                line: 2,
            },
            Token {
                typ: Return,
                lexeme: "return".to_string(),
                line: 3,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 3,
            },
            Token {
                typ: SemiColon,
                lexeme: ";".to_string(),
                line: 3,
            },
            Token {
                typ: RightBrace,
                lexeme: "}".to_string(),
                line: 4,
            },
            Token {
                typ: Return,
                lexeme: "return".to_string(),
                line: 6,
            },
            Token {
                typ: Identifier("fib".to_string()),
                lexeme: "fib".to_string(),
                line: 6,
            },
            Token {
                typ: LeftParen,
                lexeme: "(".to_string(),
                line: 6,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 6,
            },
            Token {
                typ: Minus,
                lexeme: "-".to_string(),
                line: 6,
            },
            Token {
                typ: Number(1.0),
                lexeme: "1".to_string(),
                line: 6,
            },
            Token {
                typ: RightParen,
                lexeme: ")".to_string(),
                line: 6,
            },
            Token {
                typ: Plus,
                lexeme: "+".to_string(),
                line: 6,
            },
            Token {
                typ: Identifier("fib".to_string()),
                lexeme: "fib".to_string(),
                line: 6,
            },
            Token {
                typ: LeftParen,
                lexeme: "(".to_string(),
                line: 6,
            },
            Token {
                typ: Identifier("n".to_string()),
                lexeme: "n".to_string(),
                line: 6,
            },
            Token {
                typ: Minus,
                lexeme: "-".to_string(),
                line: 6,
            },
            Token {
                typ: Number(2.0),
                lexeme: "2".to_string(),
                line: 6,
            },
            Token {
                typ: RightParen,
                lexeme: ")".to_string(),
                line: 6,
            },
            Token {
                typ: SemiColon,
                lexeme: ";".to_string(),
                line: 6,
            },
            Token {
                typ: RightBrace,
                lexeme: "}".to_string(),
                line: 7,
            },
            Token {
                typ: EOF,
                lexeme: "".to_string(),
                line: 7,
            },
        ];

        tokens
            .into_iter()
            .zip(correct)
            .for_each(|(a, b)| assert_eq!(a, b));
    }
}

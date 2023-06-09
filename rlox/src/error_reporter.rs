use exitcode::ExitCode;

use crate::token::{Token, TokenType};

pub struct ErrorReporter {
    pub had_error: bool,
    pub exit_code: Option<ExitCode>,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter {
            had_error: false,
            exit_code: None,
        }
    }

    pub fn error(&mut self, line: i32, message: &str, exit_code: ExitCode) {
        self.report(line, "", message, exit_code);
    }

    pub fn parse_error(&mut self, token: &Token, message: &str, exit_code: ExitCode) {
        if token.typ == TokenType::EOF {
            self.report(token.line, " at end", message, exit_code);
        } else {
            self.report(
                token.line,
                format!(" at '{}'", token.lexeme).as_str(),
                message,
                exit_code,
            );
        }
    }

    pub fn runtime_error(&mut self, line: i32, message: &str, exit_code: ExitCode) {
        self.report(line, "", message, exit_code);
    }

    fn report(&mut self, line: i32, _where: &str, message: &str, exit_code: ExitCode) {
        println!("[line {}] Error{}: {}", line, _where, message);
        self.had_error = true;
        self.exit_code = Some(exit_code);
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RuntimeError {
    pub line: i32,
    pub message: String,
    pub exit_code: ExitCode
}

use exitcode::{self, ExitCode};

use std::{
    env::args,
    fs,
    io::{self, Write},
};

use error_reporter::ErrorReporter;
use scanner::Scanner;

pub mod error_reporter;
pub mod scanner;
pub mod token;

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> ExitCode {
    let mut interpreter = Interpreter::default();

    let args = args().collect::<Vec<_>>();
    let args_len = args.len();

    if args_len > 2 {
        eprintln!("Usage: rlox [script]");
        return exitcode::USAGE;
    } else if args_len == 2 {
        interpreter.run_file(args[1].clone());
    } else {
        interpreter.run_prompt();
    }

    exitcode::OK
}

struct Interpreter {
    error_reporter: ErrorReporter,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            error_reporter: ErrorReporter::new(),
        }
    }
}

impl Interpreter {
    fn run_file(&mut self, file_path: String) {
        let source = fs::read_to_string(file_path).unwrap();
        self.run(source);
    }

    fn run_prompt(&mut self) {
        let mut line = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut line).unwrap();
            line = line.trim_end().to_string();

            self.run(line.clone());
            self.error_reporter.had_error = false;

            line.clear();
        }
    }

    fn run(&mut self, source: String) -> ExitCode {
        let mut scanner = Scanner::new(source, &mut self.error_reporter);
        let tokens = scanner.scan_tokens();
        println!("tokens: {:#?}", tokens);

        if self.error_reporter.had_error {
            return self.error_reporter.exit_code.unwrap();
        }

        exitcode::OK
    }
}
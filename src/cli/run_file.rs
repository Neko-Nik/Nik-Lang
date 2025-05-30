use std::fs;
use std::path::{Path, PathBuf};

use crate::{lexer::{Lexer, LexError, Token}, parser::Parser, interpreter::Interpreter};


fn check_file_is_valid(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(metadata) if metadata.is_file() && filename.ends_with(".nk") => {
            if metadata.len() > 0 {
                true
            } else {
                eprintln!("Error: File '{}' is empty", filename);
                false
            }
        }
        Ok(_) => {
            eprintln!("Error: File '{}' is not a valid script, it should end with .nk", filename);
            false
        }
        Err(_) => {
            eprintln!("Error: File '{}' does not exist", filename);
            false
        }
    }
}

fn read_file(filename: &str) -> Option<String> {
    if !check_file_is_valid(filename) {
        return None;
    }

    match fs::read_to_string(filename) {
        Ok(content) => Some(content),
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            None
        }
    }
}

fn tokenize_input(input: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<crate::parser::Stmt>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn interpret_statements(stmts: &[crate::parser::Stmt], base_path: PathBuf) -> Result<(), String> {
    let mut interpreter = Interpreter::new(base_path);
    interpreter.run(stmts).map(|_| ())
}

pub fn run_file(filename: &str) {
    if let Some(content) = read_file(filename) {
        match tokenize_input(&content) {
            Ok(tokens) => {
                // If required, log the tokens
                // for token in &tokens {
                //     println!("{:?}", token);
                // }
                match parse_tokens(tokens.clone()) {
                    Ok(stmts) => {
                        // If required, log the parsed statements
                        // for stmt in &stmts {
                        //     println!("{:?}", stmt);
                        // }

                        // Extract the directory containing the file
                        let base_path = Path::new(filename)
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .to_path_buf();

                        // Execute the statements
                        match interpret_statements(&stmts, base_path) {
                            Ok(_) => (),    // Successfully executed
                            Err(e) => eprintln!("Error executing script: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error parsing statements: {}", e),
                }
            }
            Err(e) => match e {
                LexError::UnexpectedChar(ch, line, col) => {
                    eprintln!("Unexpected character '{}' at line {}, column {}", ch, line, col);
                }
                LexError::UnterminatedString(line, col) => {
                    eprintln!("Unterminated string starting at line {}, column {}", line, col);
                }
                LexError::InvalidNumber(num, line, col) => {
                    eprintln!("Invalid number '{}' at line {}, column {}", num, line, col);
                }
            },
        }
    } else {
        eprintln!("Failed to read or validate the file '{}'", filename);
    }
}

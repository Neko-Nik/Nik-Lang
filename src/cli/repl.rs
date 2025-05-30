use rustyline::{Editor, history::FileHistory};
use rustyline::error::ReadlineError;
use std::fs;

use crate::{lexer::{Lexer, LexError, Token}, parser::Parser, interpreter::Interpreter};


fn create_history_file_if_not_exists(filename: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(filename);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(path)?;
    }
    Ok(())
}

fn tokenize_input(input: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<crate::parser::Stmt>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn run_repl() -> rustyline::Result<()> {
    println!("Welcome to Nikl REPL!");
    println!("To exit, type 'exit' or press Ctrl+D");

    let mut rl = Editor::<(), FileHistory>::new()?;
    create_history_file_if_not_exists("/tmp/.nikl_history")?;
    if !rl.load_history("/tmp/.nikl_history").is_ok() {
        eprintln!("No previous history found");
    }

    let base_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut interpreter = Interpreter::new(base_path);

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" {
                    break;
                }
                rl.add_history_entry(input)?;

                match tokenize_input(input) {
                    Ok(tokens) => {
                        // If required, get the tokens for debugging
                        // for token in &tokens {
                        //     println!("{:?}", token);
                        // }
                        match parse_tokens(tokens.clone()) {
                            Ok(stmts) => {
                                match interpreter.run(&stmts) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Runtime error: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Parse error: {}", e),
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
            }
            Err(ReadlineError::Interrupted) => {
                println!("Keyboard Interrupt");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting REPL.");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if !rl.save_history("/tmp/.nikl_history").is_ok() {
        eprintln!("Failed to save history");
    }

    Ok(())
}

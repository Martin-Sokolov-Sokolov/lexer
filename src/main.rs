use std::env;
use std::fs;
use std::fmt::Write;
use std::process;
use parser::Expr;
use parser::Parser;
use scanner::Token;
use scanner::TokenType;
mod scanner;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {

        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                String::new()
            });

            let mut buffer = String::new();
            let mut code = 0;
            let mut tokens: Vec<Token> = vec![];

            let tokenizer = scanner::Scanner::new(file_contents);

            for it in tokenizer {

                match it {
                    Ok(token) => {
                        if !token.is_empty() {
                            writeln!(buffer, "{}", token).unwrap();
                            tokens.push(token);
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        code = 65;
                    }
                }
            }

            print!("{buffer}");
            println!("EOF  null");

            if code != 0 {
                process::exit(65);
            }

        }

        "parse" => {

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                String::new()
            });

            let mut buffer = String::new();
            let mut code = 0;
            let mut tokens: Vec<Token> = vec![];

            let tokenizer = scanner::Scanner::new(file_contents);

            for it in tokenizer {

                match it {
                    Ok(token) => {
                        if !token.is_empty() {
                            writeln!(buffer, "{}", token).unwrap();
                            tokens.push(token);
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        code = 65;
                    }
                }
            }

            //print!("{buffer}");
            //println!("EOF  null");

            if code != 0 {
                process::exit(code);
            }

            tokens.push(
                Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: None, line:0 }
            );
            let parser = Parser::new(tokens);

            for it in parser {

                match it {
                    Ok(expr) => {
                        writeln!(buffer, "{}", expr).unwrap();
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(65);
                    }
                }
            }
        }
        _ => {
            return;
        }
    }
}

mod token;
mod scanner;
mod parser;
mod evaluator;
mod expr;
mod visitor;
mod stmt;
mod environment;
mod lox_callable;
mod lox_function;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::fmt::Write;
use std::process;
use std::rc::Rc;
use environment::Environment;
use parser::Parser;
use evaluator::Evaluator;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <command> <filename>");
        process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_default();
    
    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = Scanner::new(&file_contents);
            let tokens = tokenizer.scan_tokens();
            let mut buffer = String::new();
            for tok in tokens {
                writeln!(buffer, "{}", tok).unwrap();
            }
            print!("{buffer}");
            if tokenizer.code != 0 {
                process::exit(tokenizer.code);
            }
        },
        "parse" => {
            let mut tokenizer = Scanner::new(&file_contents);
            let tokens = tokenizer.scan_tokens();
            let mut parser = Parser::new(tokens);
            let res = parser.parse();
            match res {
                Ok(expr) => println!("{}", expr),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(65);
                }
            }
        },
        "evaluate" => {
            let env = Environment::new(None);
            let p_env = Rc::from(RefCell::from(env));
            let mut a = Evaluator::new(p_env);
            let mut tokenizer = Scanner::new(&file_contents);
            let tokens = tokenizer.scan_tokens();
            let mut parser = Parser::new(tokens);
            let res = parser.parse();
            match res {
                Ok(expr) => {
                    let result = a.evaluate(&expr);
                    match result {
                        Ok(_tw) => a.writer(&_tw),
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(70);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(65);
                }
            }
        }
        "run" => {
            let env = Environment::new(None);
            let p_env = Rc::from(RefCell::from(env));
            let mut a = Evaluator::new(p_env);
            let mut tokenizer = Scanner::new(&file_contents);
            let tokens = tokenizer.scan_tokens();
            let mut parser = Parser::new(tokens);
            let stmts = parser._parse();

            if let Err(e) = stmts {
                eprintln!("{e}");
                process::exit(65);
            }

            let st = stmts.unwrap();

            let _ = a.interpret(st);
            
        }
        _ => {}
    }
}

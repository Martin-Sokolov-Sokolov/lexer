use std::env;
use std::fs;
use std::fmt::Write;
use std::process;
use parser::Expr;
use parser::Parser;
use scanner::{Scanner, Token, TokenType};
mod scanner;
mod parser;
mod evaluator;
use evaluator::Evaluator;

fn tokenize(file_contents: String) -> Result<(Vec<Token>, String), String> {
    let tokenizer = Scanner::new(file_contents);
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for it in tokenizer {
        match it {
            Ok(token) if !token.is_empty() => {
                writeln!(buffer, "{}", token).unwrap();
                tokens.push(token);
            }
            Err(err) => eprintln!("{}", err.to_string()),
            _ => {}
        }
    }
    Ok((tokens, buffer))
}

fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = tokens;
    tokens.push(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: None, line: 0 });
    let mut parser = Parser::new(tokens);
    parser.parse()
}


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
        "tokenize" => match tokenize(file_contents) {
            Ok((_, buffer)) => {
                print!("{}", buffer);
                println!("EOF  null");
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(65);
            }
        },
        "parse" => match tokenize(file_contents).and_then(|(tokens, _)| parse(tokens)) {
            Ok(ast) => println!("{}", ast),
            Err(err) => {
                eprintln!("{}", err);
                process::exit(65);
            }
        },
        "evaluate" => {
            let mut a = Evaluator;
            if let Ok(expr) = tokenize(file_contents).and_then(|(tokens, _)| parse(tokens)) {
                if let Some(b) = a.evaluate(&expr) {
                    a.writer(&b);
                }
            }
        }
        _ => {}
    }
}

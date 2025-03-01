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

fn tokenize(file_contents: String) -> (Vec<Token>, String) {
    let tokenizer = Scanner::new(file_contents);
    let mut tokens = Vec::new();
    let mut err_buffer = String::new();

    for it in tokenizer {
        match it {
            Ok(token) => tokens.push(token),
            Err(err) => writeln!(err_buffer, "{}", err).unwrap(),
        }
    }
    (tokens, err_buffer)
}

fn _parse(tokens: Vec<Token>) -> Result<Expr, String> {
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
        "tokenize" => {
            let (tokens, err_buff) = tokenize(file_contents);
            let mut buffer = String::new();
            
            for tok in tokens {
                writeln!(buffer, "{}", tok).unwrap();
            }

            print!("{err_buff}");
            print!("{buffer}");
            
            if !err_buff.is_empty() {
                process::exit(65);
            }
        },
        "parse" => {
            let (tokens, _) = tokenize(file_contents);

            let op = _parse(tokens);

            match op {
                Ok(a) => println!("{}", a),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(65);
                }
            }
            
        },
        "evaluate" => {
            let mut a = Evaluator;
            let (tokens, err_buff) = tokenize(file_contents);

            if !err_buff.is_empty() {
                print!("{}", err_buff);
                process::exit(65);
            }

            let p = _parse(tokens);

            if let Ok(expr) = p {
                if let Some(val) = a.evaluate(&expr) {
                    a.writer(&val);
                }
            }
        }
        _ => {}
    }
}

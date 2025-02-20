use std::env;
use std::fs;
use std::fmt::Write;
use std::process;
mod scanner;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                String::new()
            });

            let mut buffer = String::new(); // Buffer for normal output
            let mut code = 0;

            let tokenizer = scanner::Scanner::new(file_contents);

            for it in tokenizer {

                match it {
                    Ok(token) => {
                        if !token.is_empty() {
                            writeln!(buffer, "{}", token).unwrap();
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
        _ => {
            return;
        }
    }
}

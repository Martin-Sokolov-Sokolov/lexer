use std::default;
use std::env;
use std::fs;
use std::io::{self, Write};

fn work_with_parenthesis(contents: &str) -> String {
    let mut res = String::new();

    for ch in contents.chars() {
        match ch {
            '(' => res += "LEFT_PAREN ( null",
            ')' => res += "RIGHT_PAREN ) null",
            _ => (),
        }
    }

    res += "EOF null";

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                panic!("Scanner not implemented");
            } else {
                let pars = work_with_parenthesis(&file_contents);
                println!("{pars} 123");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

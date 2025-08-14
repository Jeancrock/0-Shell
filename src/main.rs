mod parser;
mod utils;
mod builtins;

use std::io::{self, Write};
use builtins::*;

fn main() {
    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush().ok();

        // Lecture ligne
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break; // EOF / Ctrl+D
            }
            Ok(_) => {
                let args = parser::parse_args(&input);
                if args.is_empty() { continue; }

                if let Err(e) = dispatch(&args) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                eprintln!("readline: {}", e);
                break;
            }
        }
    }
}

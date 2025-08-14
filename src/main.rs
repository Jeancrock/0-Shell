mod builtins;
mod parser;
mod utils;

use std::io::{self, Write};

use builtins::dispatch;

fn main() {
    // Program loop
    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush().ok();

        // Input reader
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            // EOF / Ctrl+D
            Ok(0) => {
                println!();
                break;
            }

            // Read args
            Ok(_) => {
                let args = parser::parse_args(&input);
                if args.is_empty() {
                    continue;
                }

                // Execute command function
                match dispatch(&args) {

                    // Continue
                    Ok(true) => continue,

                    // Exit
                    Ok(false) => break,

                    // Error
                    Err(e) => eprintln!("{}", e),
                }
            }

            // Error
            Err(e) => {
                eprintln!("readline: {}", e);
                break;
            }
        }
    }
}

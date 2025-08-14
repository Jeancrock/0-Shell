mod builtins;
mod parser;
mod utils;

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use whoami::{self, fallible};

fn main() {
    let mut history: Vec<String> = Vec::new();
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    loop {
        // Récupérer username@hostname
        let username = whoami::username();
        let hostname = fallible::hostname().unwrap_or_else(|_| "unknown".to_string());

        // Récupérer le répertoire courant
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
        let display_cwd = if cwd.starts_with(&home_dir) {
            // Remplacer le chemin du home par ~
            let mut p = PathBuf::from("~");
            p.push(cwd.strip_prefix(&home_dir).unwrap());
            p.display().to_string()
        } else {
            cwd.display().to_string()
        };

        // Afficher le prompt : username@hostname:cwd$
        print!(
            "\x1b[90m◯ \x1b[37m(0-Shell) \x1b[32m{}@{}\x1b[37m:\x1b[34m{}\x1b[37m$ \x1b[0m",
            username, hostname, display_cwd
        );
        io::stdout().flush().ok();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                let input = input.trim_end().to_string();
                if input.is_empty() {
                    continue;
                }

                let args = parser::parse_args(&input);
                if args.is_empty() {
                    continue;
                }

                match builtins::dispatch(&args, &history) {
                    Ok(true) => {
                        if args[0] != "history" {
                            history.push(input.clone());
                        }
                        continue;
                    }
                    Ok(false) => break,
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => {
                eprintln!("readline: {}", e);
                break;
            }
        }
    }
}

mod builtins;
mod parser;
mod utils;

use std::io::{self, Write};
use std::path::PathBuf;
use std::env;

use whoami::{self, fallible};

use builtins::dispatch;

fn main() {
    let mut history: Vec<String> = Vec::new();
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    loop {
        // Récupérer username@hostname
        let username = whoami::username();
        let hostname = fallible::hostname().unwrap_or_else(|_| "unknown".to_string());

        // Récupérer cwd
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
        let display_cwd = if cwd.starts_with(&home_dir) {
            let mut p = PathBuf::from("~");
            p.push(cwd.strip_prefix(&home_dir).unwrap());
            p.display().to_string()
        } else {
            cwd.display().to_string()
        };

        // Prompt
        print!(
            "\x1b[90m◯ \x1b[37m(0-Shell) \x1b[32m{}@{}\x1b[37m:\x1b[34m{}\x1b[37m$ \x1b[0m",
            username, hostname, display_cwd
        );
        io::stdout().flush().ok();

        // Lire l'entrée utilisateur
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            // EOF (Ctrl+D)
            Ok(0) => {
                println!();
                break;
            }

            // Nouvelle ligne lue
            Ok(_) => {
                // Nettoyer l'entrée
                let input = input.trim_end().to_string();
                if input.is_empty() {
                    continue;
                }

                // Parser les arguments
                let args = parser::parse_args(&input);
                if args.is_empty() {
                    continue;
                }

                // Dispatch de la commande
                match dispatch::dispatch(&args, &history) {
                    Ok(true) => {
                        if args[0] != "history" {
                            history.push(input.clone());
                        }
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

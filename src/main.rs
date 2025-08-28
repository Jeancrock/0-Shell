mod builtins;
mod parser;
mod utils;

use std::{env, process};
use std::io::{self, Write};
use std::path::PathBuf;

use whoami::{self, fallible};
use builtins::dispatch;

fn main() {
    let mut history: Vec<String> = Vec::new();
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));

    loop {
        // Prompt username@hostname:cwd
        let username = whoami::username();
        let hostname = fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
        let display_cwd = if cwd.starts_with(&home_dir) {
            let mut p = PathBuf::from("~");
            p.push(cwd.strip_prefix(&home_dir).unwrap());
            p.display().to_string()
        } else {
            cwd.display().to_string()
        };
        print!(
            "\x1b[90m◯ \x1b[37m(0-Shell) \x1b[32m{}@{}\x1b[37m:\x1b[34m{}\x1b[37m$ \x1b[0m",
            username, hostname, display_cwd
        );
        io::stdout().flush().ok();

        // Lire entrée utilisateur
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                process::exit(0);
            }
            Ok(_) => {
                let input = input.trim_end().to_string();
                if input.is_empty() {
                    continue;
                }

                let mut args = parser::parse_args(&input);
                if args.is_empty() {
                    continue;
                }

                // Détecter le &
                let mut background = false;
                if args.last() == Some(&"&".to_string()) {
                    background = true;
                    args.pop();
                }

                if background {
                    // Background : lancer un nouveau processus pour cette commande
                    let args_clone = args.clone();
                    let input_clone = input.clone();
                    std::thread::spawn(move || {
                        // Exécuter la commande interne dans un processus enfant
                        if let Err(e) = dispatch::dispatch(&args_clone, &Vec::new()) {
                            eprintln!("{}", e);
                        }
                        println!("Processus lancé en arrière-plan : {}", input_clone);
                    });
                } else {
                    // Foreground : exécuter directement et mettre à jour history
                    match dispatch::dispatch(&args, &history) {
                        Ok(true) => {
                            if args[0] != "history" {
                                history.push(input.clone());
                            }
                        }
                        Ok(false) => process::exit(0),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("readline: {}", e);
                break;
            }
        }
    }
}

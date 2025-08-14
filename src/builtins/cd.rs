use std::{env, path::PathBuf};

pub fn cmd_cd(args: &[String]) {
    let target = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };

    let mut path = PathBuf::new();

    if target.starts_with('/') {
        path.push(&target);
    } else {
        match env::current_dir() {
            Ok(cwd) => {
                path.push(cwd);
                path.push(&target);
            }
            Err(e) => {
                eprintln!("cd: erreur en obtenant le répertoire courant: {}", e);
                return;
            }
        }
    }

    match path.canonicalize() {
        Ok(resolved) => {
            if !resolved.exists() {
                eprintln!("{}: No such file or directory", target);
                return;
            }
            if let Err(e) = env::set_current_dir(&resolved) {
                eprintln!("{}: {}", target, e);
            }
        }
        Err(_) => {
            eprintln!("{}: No such file or directory", target);
        }
    }
}

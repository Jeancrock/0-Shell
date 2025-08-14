use chrono::{DateTime, Local};
use std::{fs, os::unix::fs::MetadataExt, path::Path};

use crate::utils;

#[derive(Default)]
struct LsFlags {
    long: bool,
    all: bool,
    classify: bool,
}

pub fn cmd_ls(args: &[String]) -> Result<(), String> {
    let mut flags = LsFlags::default();
    let mut paths = Vec::new();

    // Parsing des options
    for a in &args[1..] {
        if a.starts_with('-') {
            for ch in a.chars().skip(1) {
                match ch {
                    'l' => flags.long = true,
                    'a' => flags.all = true,
                    'F' => flags.classify = true,
                    _ => return Err(format!("ls: invalid option -- '{}'", ch)),
                }
            }
        } else {
            paths.push(a.clone());
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for p in paths {
        let path = Path::new(&p);
        let mut entries = utils::read_and_collect_dir(path)
            .map_err(|e| format!("ls: cannot access '{}': {}", p, e))?;

        // Ajouter '.' et '..' si -a
        if flags.all {
            if let Ok(meta_dot) = fs::metadata(path) {
                entries.insert(0, (".".to_string(), meta_dot));
            }
            if let Ok(meta_dotdot) = fs::metadata(path.join("..")) {
                entries.insert(1, ("..".to_string(), meta_dotdot));
            }
        }

        // Tri alphabétique en ignorant le '.' initial pour fichiers cachés
        entries.sort_by(|a, b| {
            let a_name = if a.0.starts_with('.') {
                &a.0[1..]
            } else {
                &a.0
            };
            let b_name = if b.0.starts_with('.') {
                &b.0[1..]
            } else {
                &b.0
            };
            a_name.to_lowercase().cmp(&b_name.to_lowercase())
        });

        if flags.long {
            // Affichage total comme ls -l
            let total_blocks: u64 = entries
                .iter()
                .filter(|(name, _)| flags.all || !name.starts_with('.'))
                .map(|(_, md)| (md.blocks() * 512 + 1023) / 1024) // en Kio
                .sum();
            println!("total {}", total_blocks);

            for (name, meta) in &entries {
                if !flags.all && name.starts_with('.') {
                    continue;
                }

                let permissions = utils::format_permissions(&meta);
                let nlink = meta.nlink();
                let size = meta.len();
                let mtime: DateTime<Local> = DateTime::from(meta.modified().unwrap());
                let formatted_date = mtime.format("%b %e %H:%M").to_string();

                println!(
                    "{} {:>2} {:<8} {:<8} {:>6} {} {}",
                    permissions,
                    nlink,
                    whoami::username(),
                    whoami::username(),
                    size,
                    formatted_date,
                    utils::format_name(name, meta, flags.classify)
                );
            }
        } else {
            let mut line = String::new();
            for (name, meta) in &entries {
                if !flags.all && name.starts_with('.') {
                    continue;
                }
                let formatted = utils::format_name(name, meta, flags.classify);
                line.push_str(&formatted);
                line.push(' ');
            }
            println!("{}", line.trim_end());
        }
    }

    Ok(())
}

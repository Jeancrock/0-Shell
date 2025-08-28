use chrono::{DateTime, Local};
use std::{fs, os::unix::fs::MetadataExt, path::Path};
use whoami::fallible;

use crate::utils;

#[derive(Default)]
struct LsFlags {
    long: bool,      // <- option -l
    all: bool,       // <- option -a
    classify: bool,  // <- option -F
    recursive: bool, // <- option -R
    reverse: bool,   // <- option -r
}

/// Commande `ls`
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
                    'R' => flags.recursive = true,
                    'r' => flags.reverse = true,
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
        // On capture l'erreur principale ici, mais on continue avec les autres chemins
        if let Err(e) = ls_directory(path, &flags) {
            eprintln!("{}", e);
        }
    }

    Ok(())
}

/// Fonction qui liste un répertoire (et descend si -R est activé)
fn ls_directory(path: &Path, flags: &LsFlags) -> Result<(), String> {
    let mut entries = match utils::read_and_collect_dir(path) {
        Ok(e) => e,
        Err(e) => return Err(format!("ls: cannot access '{}': {}", path.display(), e)),
    };

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
        fn first_alnum_index(s: &str) -> usize {
            s.char_indices()
                .find(|(_, c)| c.is_alphanumeric())
                .map(|(i, _)| i)
                .unwrap_or(0)
        }

        let a_start = first_alnum_index(&a.0);
        let b_start = first_alnum_index(&b.0);

        a.0[a_start..]
            .to_lowercase()
            .cmp(&b.0[b_start..].to_lowercase())
    });

    // Inversion si -r
    if flags.reverse {
        entries.reverse();
    }

    // Mode long (-l)
    if flags.long {
        let total_blocks: u64 = entries
            .iter()
            .filter(|(name, _)| flags.all || !name.starts_with('.'))
            .map(|(_, md)| (md.blocks() * 512 + 1023) / 1024) // en Kio
            .sum();
        println!("total {}", total_blocks);

        let max_digits_nlink = entries
            .iter()
            .map(|(_, meta)| meta.nlink())
            .max()
            .unwrap_or(0)
            .to_string()
            .len();

        let max_digits_size = entries
            .iter()
            .map(|(_, meta)| meta.len())
            .max()
            .unwrap_or(0)
            .to_string()
            .len();

        for (name, meta) in &entries {
            if !flags.all && name.starts_with('.') {
                continue;
            }

            let permissions = utils::format_permissions(&meta);
            let nlink = meta.nlink();
            let size = meta.len();
            let mtime: DateTime<Local> = DateTime::from(meta.modified().unwrap());
            let formatted_date = mtime.format("%b  %e %H:%M").to_string();

            println!(
                "{} {:>max_digits_nlink$} {:<8}{:<8}{:>max_digits_size$} {} {}",
                permissions,
                nlink,
                whoami::username(),
                fallible::hostname().unwrap_or_else(|_| "unknown".to_string()),
                size,
                formatted_date,
                utils::format_name(name, meta, flags.classify),
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

    // Si -R : parcourir récursivement les sous-dossiers
    if flags.recursive {
        for (name, meta) in &entries {
            if !flags.all && name.starts_with('.') {
                continue;
            }
            if meta.is_dir() && name != "." && name != ".." {
                println!();
                // ⚠️ Capture l'erreur pour continuer même si le dossier n'est pas accessible
                if let Err(e) = ls_directory(&path.join(name), flags) {
                    eprintln!("{}", e); // affiche l’erreur mais ne quitte pas
                    continue;
                }
            }
        }
    }

    Ok(())
}

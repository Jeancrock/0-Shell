use std::fs;
use std::io;
use std::path::Path;

pub fn cmd_mkdir(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: mkdir [-p] <folder1> [folder2 ...]");
        return Ok(());
    }

    let mut start = 1;
    let recursive = if args[1] == "-p" {
        start = 2;
        true
    } else {
        false
    };

    for folder in &args[start..] {
        let path = Path::new(folder);
        if recursive {
            fs::create_dir_all(path)?; // crée tous les parents nécessaires
        } else {
            fs::create_dir(path)?; // crée uniquement le dossier direct
        }
    }

    Ok(())
}

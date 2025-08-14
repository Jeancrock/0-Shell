use std::{fs, path::Path};

pub fn cmd_mv(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let src = Path::new(&args[1]);
    let dst = Path::new(&args[2]);

    if !src.exists() {
        eprintln!("mv: cannot stat '{}': No such file or directory", src.display());
        return;
    }

    // Si la destination est un répertoire existant → placer le fichier/dossier dedans
    let dst_final = if dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };

    if let Err(e) = fs::rename(&src, &dst_final) {
        eprintln!("mv: failed to move '{}': {}", src.display(), e);
    }
}

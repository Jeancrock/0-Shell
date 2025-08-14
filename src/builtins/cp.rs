use std::{fs, path::Path};

pub fn cmd_cp(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let src = Path::new(&args[1]);
    let dst = Path::new(&args[2]);

    if !src.exists() {
        eprintln!("cp: cannot stat '{}': No such file or directory", src.display());
        return;
    }

    // Si la destination est un dossier → y mettre le même nom de fichier
    let dst_final = if dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };

    if let Err(e) = fs::copy(&src, &dst_final) {
        eprintln!("cp: failed to copy '{}': {}", src.display(), e);
    }
}

use std::fs;

pub fn cmd_rm(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("rm: missing operand".to_string());
    }
    let mut recursive = false;
    let mut files = Vec::new();

    for arg in &args[1..] {
        if arg == "-r" {
            recursive = true;
        } else {
            files.push(arg.clone());
        }
    }

    for file in files {
        let path = std::path::Path::new(&file);
        if recursive {
            fs::remove_dir_all(path).map_err(|e| format!("rm: {}: {}", file, e))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("rm: {}: {}", file, e))?;
        }
    }
    Ok(())
}

use crate::utils;
use std::path::Path;

#[derive(Default)]
struct LsFlags {
    long: bool,
    all: bool,
    classify: bool,
}

pub fn cmd_ls(args: &[String]) -> Result<(), String> {
    let mut flags = LsFlags::default();
    let mut paths = Vec::new();

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
        let entries = utils::read_and_collect_dir(path).map_err(|e| format!("ls: {}: {}", p, e))?;

        for (name, meta) in entries {
            if !flags.all && name.starts_with('.') {
                continue;
            }
            if flags.long {
                utils::print_long_line(path.join(&name), &meta, flags.classify)?;
            } else {
                println!("{}", utils::format_name(&name, &meta, flags.classify));
            }
        }
    }
    Ok(())
}

use std::fs;

pub fn cmd_cp(args: &[String]) -> Result<(), String> {
    if args.len() != 3 {
        return Err("cp: usage: cp <source> <destination>".to_string());
    }
    fs::copy(&args[1], &args[2])
        .map_err(|e| format!("cp: {} -> {}: {}", args[1], args[2], e))?;
    Ok(())
}

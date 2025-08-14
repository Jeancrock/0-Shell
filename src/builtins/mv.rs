use std::fs;

pub fn cmd_mv(args: &[String]) -> Result<(), String> {
    if args.len() != 3 {
        return Err("mv: usage: mv <source> <destination>".to_string());
    }
    fs::rename(&args[1], &args[2])
        .map_err(|e| format!("mv: {} -> {}: {}", args[1], args[2], e))?;
    Ok(())
}

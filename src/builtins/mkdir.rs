use std::fs;

pub fn cmd_mkdir(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("mkdir: missing operand".to_string());
    }
    for dir in &args[1..] {
        fs::create_dir(dir).map_err(|e| format!("mkdir: {}: {}", dir, e))?;
    }
    Ok(())
}

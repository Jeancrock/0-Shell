use std::io::Write;

pub fn cmd_clear() -> Result<(), String> {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(())
}

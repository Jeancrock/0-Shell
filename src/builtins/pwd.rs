use std::env;

pub fn cmd_pwd(_args: &[String]) -> Result<(), String> {
    let path = env::current_dir().map_err(|e| format!("pwd: {}", e))?;
    println!("{}", path.display());
    Ok(())
}

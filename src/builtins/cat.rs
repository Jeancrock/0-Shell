use std::fs;

pub fn cmd_cat(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cat: missing operand".to_string());
    }
    for file in &args[1..] {
        match fs::read_to_string(file) {
            Ok(content) => println!("{}", content),
            Err(e) => return Err(format!("cat: {}: {}", file, e)),
        }
    }
    Ok(())
}

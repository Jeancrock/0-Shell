use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

pub fn cmd_write(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: write <filename>");
        return Ok(());
    }
    let filename = &args[1];
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;
    println!("Enter lines to append (Ctrl+D to finish):");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        write!(file, "{}", line?)?;
    }
    Ok(())
}

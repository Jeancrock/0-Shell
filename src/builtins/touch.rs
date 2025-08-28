use std::fs::OpenOptions;
use std::io;

pub fn cmd_touch(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: touch <filename>");
        return Ok(());
    }

    for filename in &args[1..] {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(filename)?;
    }
    Ok(())
}

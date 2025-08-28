use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn cmd_echo(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        println!();
        return Ok(());
    }

    // Chercher une redirection '>' ou '>>'
    let mut redirect = None;
    let mut content = Vec::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            ">" => {
                if i + 1 < args.len() {
                    redirect = Some((args[i + 1].clone(), false)); // false = écraser
                    i += 1;
                } else {
                    eprintln!("echo: syntax error near >");
                    return Ok(());
                }
            }
            ">>" => {
                if i + 1 < args.len() {
                    redirect = Some((args[i + 1].clone(), true)); // true = append
                    i += 1;
                } else {
                    eprintln!("echo: syntax error near >>");
                    return Ok(());
                }
            }
            s => content.push(s),
        }
        i += 1;
    }

    let output = content.join(" ");

    if let Some((file, append)) = redirect {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(append)
            .truncate(!append)
            .open(file)?;
        write!(f, "{}", output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

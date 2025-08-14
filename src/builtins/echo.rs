pub fn cmd_echo(args: &[String]) -> Result<(), String> {
    if args.len() > 1 {
        println!("{}", args[1..].join(" "));
    } else {
        println!();
    }
    Ok(())
}

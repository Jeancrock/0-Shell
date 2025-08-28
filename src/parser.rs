pub fn parse_args(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut buf = String::new();
    let mut in_quotes = false;

    for ch in line.trim().chars() {
        match ch {
            // Handle quotes to allow spaces within quoted strings
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !buf.is_empty() {
                    args.push(buf.clone());
                    buf.clear();
                }
            }
            // Handle newlines and carriage returns outside of quotes
            '\n' | '\r' if !in_quotes => {
                if !buf.is_empty() {
                    args.push(buf.clone());
                    buf.clear();
                }
            }
            _ => buf.push(ch),
        }
    }

    if !buf.is_empty() {
        args.push(buf);
    }
    args
}

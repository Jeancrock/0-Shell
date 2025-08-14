/// parse_args : sépare une ligne en arguments en gérant les guillemets doubles
pub fn parse_args(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut buf = String::new();
    let mut in_quotes = false;

    for ch in line.trim().chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !buf.is_empty() {
                    args.push(buf.clone());
                    buf.clear();
                }
            }
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

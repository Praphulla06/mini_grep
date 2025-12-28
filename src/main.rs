use mini_grep::{find_lines_with_pattern, parse_input};

fn main() {
    let (pattern, mut file) = match parse_input() {
        Ok((p, f)) => (p, f),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    for line in find_lines_with_pattern(pattern.as_str(), &mut file) {
        println!("{}", line);
    }
    println!("{}", pattern);
    println!("{:#?}", file);
}

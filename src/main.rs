use mini_grep::parse_input;

fn main() {
    let (pattern, file) = match parse_input() {
        Ok((p, f)) => (p, f),
        Err(e) => {
            println!("{}", e);
            return; 
        }
    };

    println!("{}", pattern);
    println!("{:#?}", file);
}
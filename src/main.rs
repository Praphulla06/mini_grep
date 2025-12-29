use mini_grep::parse_args;

fn main() {
    let config = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("{:#?}", config);
}

use std::env;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

pub fn parse_input() -> Result<(String, File), io::Error> {
    let args: Vec<String> = env::args().collect();

    let mut pattern_index = 1;
    let mut file_index = 2;

    if let Some(input) = args.get(1) {
        if input.starts_with("_") {
            parse_flags(&args);
            pattern_index = 2;
            file_index = 3;
        }
    }
    let pattern = match args.get(pattern_index) {
        Some(p) => p.to_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid Input: Enter a pattern!",
            ));
        }
    };
    let file: File = match args.get(file_index) {
        Some(filename) => match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                println!("File index: {}", file_index);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "Invalid Input: Enter a filename",
            ));
        }
    };
    println!("File index: {}", pattern_index);
    println!("File index: {}", file_index);
    Ok((pattern, file))
}

fn parse_flags(args: &Vec<String>) {
    let flag: String = match args.get(2) {
        Some(flag) => match flag.as_str() {
            "_i" => format!("Case insensitive matching"),
            "_v" => format!("Invert match"),
            "_n" => format!("Show line number"),
            "_w" => format!("Whole word matching"),
            "_o" => format!("Print only matching part"),
            _ => format!("Invalid flag"),
        },
        None => String::new(),
    };
}

pub fn find_lines_with_pattern(pattern: &str, file: &mut File) -> Vec<String> {
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| line.contains(pattern))
        .collect()
}

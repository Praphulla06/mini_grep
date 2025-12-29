use std::io;

#[derive(Debug)]
pub struct Config {
    pattern: String,
    filenames: Vec<String>,
    ignore_case: bool,
    invert_match: bool,
    line_number: bool,
    whole_word: bool,
    only_matching: bool,
}
pub fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Err("Usage: mini_grep [flags] pattern file1 [file2 ...]".to_string());
    }

    let mut config = Config {
        pattern: String::new(),
        filenames: Vec::new(),
        ignore_case: false,
        invert_match: false,
        line_number: false,
        whole_word: false,
        only_matching: false,
    };

    let mut args_iter = args.iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-i" => config.ignore_case = true,
            "-v" => config.invert_match = true,
            "-n" => config.line_number = true,
            "-w" => config.whole_word = true,
            "-o" => config.only_matching = true,
            _ if config.pattern.is_empty() => config.pattern = arg.clone(),
            _ => config.filenames.push(arg.clone()),
        }
    }

    if config.pattern.is_empty() || config.filenames.is_empty() {
        return Err("Pattern or filename missing".to_string());
    }

    Ok(config)
}

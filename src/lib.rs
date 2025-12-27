use std::{env, fs::File, io};

pub fn parse_input() -> Result<(String, File), io::Error> {
    let args: Vec<String> = env::args().collect();

    let pattern = match args.get(1) {
        Some(p) => p.to_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Enter a pattern!",
            ));
        }
    };

    let file: File = match args.get(2) {
        Some(filename) => match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "Enter a filename",
            ));
        }
    };

    Ok((pattern, file))
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

fn collect_lines_from_file(file: &str) -> io::Result<Vec<String>> {
    let path = PathBuf::from(file);
    let reader = BufReader::new(File::open(path)?);
    reader.lines().collect()
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        eprintln!("Usage: lines_result FILE");
        std::process::exit(1);
    }

    if let Ok(lines) = collect_lines_from_file(&args[0]) {
        for line in lines {
            println!("{}", line);
        }
    }
}
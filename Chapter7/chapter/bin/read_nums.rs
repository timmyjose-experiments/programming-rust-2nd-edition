use std::fs::File;
use std::io::{BufRead, BufReader};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = Vec::new();
    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn main() {
    let mut input = BufReader::new(File::open("./numbers.in").unwrap());
    match read_numbers(&mut input) {
        Err(e) => eprintln!("{}", e),
        Ok(numbers) => {
            println!("{}", numbers.iter().sum::<i64>());
            println!("{}", numbers.iter().product::<i64>());
        }
    }
}

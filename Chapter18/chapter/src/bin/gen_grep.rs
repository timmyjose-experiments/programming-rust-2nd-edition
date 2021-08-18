use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn grep<R>(target: &str, reader: &mut R) -> io::Result<()>
where
    R: BufRead,
{
    for line in reader.lines() {
        let line = line?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    grep("the", &mut stdin_lock).unwrap();
    println!();

    grep("the", &mut BufReader::new(File::open("grep.txt").unwrap())).unwrap();
}

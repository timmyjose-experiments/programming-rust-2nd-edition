use std::io::{self, BufRead};

fn grep(target: &str) -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.contains(target) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn main() {
    grep("the").unwrap();
}

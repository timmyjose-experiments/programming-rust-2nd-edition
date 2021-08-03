use std::fs::File;
use std::io::{Result, Write};

fn say_hello(out: &mut dyn Write) -> Result<()> {
    out.write_all(b"Hello, world!")?;
    out.flush()
}

fn main() -> Result<()> {
    let mut v = Vec::new();
    say_hello(&mut v)?;
    println!("{:?}", v);

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    Ok(())
}
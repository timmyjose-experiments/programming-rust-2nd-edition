use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

fn demo() -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin
            .write_all("Hello, world!".as_bytes())
            .expect("failed to write to stdin");
    });

    let output = child.wait_with_output()?;
    assert_eq!(String::from_utf8_lossy(&output.stdout), "!dlrow ,olleH\n");

    Ok(())
}

fn main() {
    match demo() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
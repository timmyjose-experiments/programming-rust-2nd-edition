use std::error::Error;
use std::fmt;
use std::io::{stderr, Write};

#[derive(Debug)]
struct SuperError {
    side: SuperErrorSidekick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct SuperErrorSidekick;

impl fmt::Display for SuperErrorSidekick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSidekick is here!")
    }
}

impl Error for SuperErrorSidekick {}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError {
        side: SuperErrorSidekick,
    })
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "Caused by: {}", source);
        err = source;
    }
}

fn main() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e);
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    }

    let err = get_super_error().err().unwrap();
    print_error(&err);
}

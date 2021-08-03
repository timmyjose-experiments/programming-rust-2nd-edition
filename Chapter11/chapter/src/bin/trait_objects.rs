use std::io::{Result, Write};

fn say_hello(out: &mut dyn Write) -> Result<()> {
    out.write_all(b"Hello again!")?;
    out.flush()
}

fn say_hola<W: Write>(out: &mut W) -> Result<()> {
    out.write_all(b"Hola, mundo!")?;
    out.flush()
}

fn main() -> std::io::Result<()> {
    let mut buf = Vec::new();
    let writer: &mut dyn Write = &mut buf;
    println!(
        "sizeof writer = {} bytes",
        std::mem::size_of::<&mut dyn Write>()
    );
    writer.write_all(b"Hello, world!")?;

    say_hello(&mut buf);
    say_hola(&mut buf);

    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

fn copy_file<R, W>(writer: &mut W, reader: &mut R) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    for line in reader.lines() {
        // this preferable to using the low-level API
        //writeln!(writer, "{}", line?)?;
        writer.write_all(line?.as_bytes())?;
    }
    writer.flush()?;

    Ok(())
}

fn copy_main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("{}", "Usage: copy_file TARGET SOURCE");
        std::process::exit(1);
    }

    let mut src = BufReader::new(File::open(PathBuf::from(&args[0]))?);
    let mut dst = BufWriter::new(File::create(PathBuf::from(&args[1]))?);

    copy_file(&mut dst, &mut src)?;

    Ok(())
}

fn main() {
    if let Err(err) = copy_main() {
        eprintln!("Something went wrong: {}", err);
        std::process::exit(1);
    } else {
        println!("Copy successful.");
    }
}
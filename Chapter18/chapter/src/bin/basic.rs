use std::io::{self, BufRead, ErrorKind, Read, Write};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn copy<R: ?Sized + Read, W: ?Sized + Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

fn main() {
    let src = vec![1, 2, 3, 4, 5];
    //let dst = Vec::new();
    //copy(&mut src as &mut dyn Read, &mut dst);
    //println!("{:?}", dst);

    for line in src.lines() {
        println!("{:?}", line);
    }
}
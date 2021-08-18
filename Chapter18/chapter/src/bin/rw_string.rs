use std::io::{self, Cursor, ErrorKind, Read, Write};

const BUF_SIZE: usize = 8 * 1024;

fn read_from_string(s: String) -> io::Result<()> {
    let mut cursor = Cursor::new(s);

    let mut buf = [0; BUF_SIZE];

    loop {
        match cursor.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    }
}

fn main() {
    let text = "Hello, world\nNice to meet you again\nAdios, muchachos!".to_string();
    read_from_string(text).unwrap();
}
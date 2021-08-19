use std::io;
use std::net::TcpListener;
use std::thread;

fn echo(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on address: {}", addr);

    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("Connection accepted from {:?}", addr);

        let mut write_stream = stream.try_clone()?;
        thread::spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("error in client thread");
            println!("Connection closed");
        });
    }
}

fn main() {
    echo("127.0.0.1:9999").unwrap();
}
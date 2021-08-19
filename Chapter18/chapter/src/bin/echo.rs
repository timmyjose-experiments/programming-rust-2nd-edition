use std::io;
use std::net::TcpListener;
use std::thread::spawn;

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);

    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("Connection received from {}", addr);

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            println!("Connection closed");
        });
    }
}

fn main() {
    echo_main("127.0.0.1:9999").expect("error: ");
}
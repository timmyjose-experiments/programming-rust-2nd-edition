use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("Hello from thread");
    });

    handle
        .join()
        .expect("something went wrong while waiting for the thread to finish");
}
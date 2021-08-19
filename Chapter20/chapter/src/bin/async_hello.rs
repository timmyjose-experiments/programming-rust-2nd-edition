use async_std::task;
use std::thread;
use std::time::Duration;

async fn hello() {
    thread::sleep(Duration::from_secs(2));
    println!("Hello, world!");
}

fn main() {
    task::block_on(hello());
    println!("hello");
    thread::sleep(Duration::from_secs(2));
    println!("world");
}
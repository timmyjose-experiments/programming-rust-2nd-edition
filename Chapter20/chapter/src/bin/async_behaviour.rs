use async_std::task;
use std::time::{Duration, SystemTime};

async fn say_hello() {
    println!("Hello, I was launched at {:?}!", SystemTime::now());
    task::sleep(Duration::from_secs(5)).await;
}

async fn say_privet() {
    println!("Privet, I was launched at {:?}!", SystemTime::now());
    task::sleep(Duration::from_secs(1)).await;
}

async fn say_hola() {
    println!("Hola, I was launched at {:?}!", SystemTime::now());
    task::sleep(Duration::from_secs(2)).await;
}

fn foo() {
    println!("Hello from foo!");
}

async fn launch_tasks() {
    // this is fully sequential!
    foo();
    say_hello().await;
    say_privet().await;
    say_hola().await;
}

fn main() {
    task::block_on(launch_tasks());
}

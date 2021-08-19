use async_std::task;
use std::time::{Duration, SystemTime};

async fn say_hello() {
    println!("Hello, I was called at {:?}", SystemTime::now());
    let mut counter = 0;
    while counter != 5 {
        task::yield_now().await;
        task::sleep(Duration::from_secs(1)).await;
        counter += 1;
    }
}

async fn say_privet() {
    println!("Privet, I was called at {:?}", SystemTime::now());
    let mut counter = 0;
    while counter != 2 {
        task::sleep(Duration::from_secs(2)).await;
        task::yield_now().await;
        counter += 1;
    }
}

async fn say_hola() {
    println!("Hola, I was called at {:?}", SystemTime::now());
    task::sleep(Duration::from_secs(1)).await;
}

async fn handle_calls() {
    // this makes it non-sequential!
    let handles = vec![
        task::spawn_local(say_hello()),
        task::spawn_local(say_privet()),
        task::spawn_local(say_hola()),
    ];

    for handle in handles {
        handle.await;
    }
}

fn main() {
    task::block_on(handle_calls());
}

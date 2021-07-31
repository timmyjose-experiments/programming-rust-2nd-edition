use std::panic;

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let share = panic::catch_unwind(|| {
        let half = total / 2;
        half / crew_size as u64
    });

    if share.is_err() {
        0
    } else {
        share.unwrap()
    }
}

fn main() {
    let result = panic::catch_unwind(|| {
        println!("Hello");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("something went wrong!");
    });
    assert!(result.is_err());

    println!("{}", pirate_share(1024, 11));
    println!("{}", pirate_share(1024, 0));
}

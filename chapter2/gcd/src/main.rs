use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        usage();
    }

    let mut g = numbers[0];
    for n in &numbers[1..] {
        g = gcd::gcd(g, *n);
    }

    println!("The gcd of {:#?} is {}", numbers, g);
}

fn usage() {
    println!("Usage: gcd number number*");
    std::process::exit(0);
}

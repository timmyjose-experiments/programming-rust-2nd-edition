use std::str::FromStr;

fn usage() {
    eprintln!("Usage: calc NUM-1 NUM-2");
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 2 {
        usage();
    }

    let (x, y) = (
        i32::from_str(&args[0]).expect("could not parse the first number"),
        i32::from_str(&args[1]).expect("could not parse the second number"),
    );
    println!("{} + {} = {}", x, y, add::add(x, y));
    println!("{} - {} = {}", x, y, subtract::subtract(x, y));
    println!("{} * {} = {}", x, y, multiply::multiply(x, y));
    println!("{} / {} = {}", x, y, divide::divide(x, y));
}

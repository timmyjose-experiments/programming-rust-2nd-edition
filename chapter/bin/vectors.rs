use std::fmt::Display;

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    assert_eq!(nums.iter().product::<i32>(), 120);

    nums.push(6);
    nums.push(7);
    assert_eq!(nums.iter().product::<i32>(), 5040);

    let pixels = vec![0u8; 1024];
    assert_eq!(pixels.len(), 1024);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);
    assert!(is_palindrome(&pal.iter().fold(
        String::new(),
        |mut acc, &s| {
            acc.push_str(s);
            acc
        }
    )));

    let evens = (1..=10).filter(|&n| n % 2 == 0).collect::<Vec<_>>();
    let odds = (1..=10).filter(|&n| n % 2 == 1).collect::<Vec<_>>();
    println!("{:#?}\n{:#?}", odds, evens);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    print(&nums);
    print(&[11, 12, 13, 14, 15]);
}

fn is_palindrome(s: &str) -> bool {
    let mut t = s.chars().rev();
    for c in s.chars() {
        if c != t.next().unwrap() {
            return false;
        }
    }
    true
}

fn print<T: Display>(slice: &[T]) {
    for e in slice {
        println!("{}", e);
    }
}

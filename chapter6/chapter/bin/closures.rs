fn main() {
    let is_even = |n| n % 2 == 0;
    assert!(is_even(2));
    assert!(!is_even(3));

    let is_odd = |n: i32| -> bool { n % 2 == 1 };
    assert!(is_odd(21));
    assert!(!is_odd(32));
}

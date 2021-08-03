fn min<T: Ord>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

fn main() {
    assert_eq!(min::<i32>(10, 12), 10);
    assert_eq!(min(10, 12), 10);
}

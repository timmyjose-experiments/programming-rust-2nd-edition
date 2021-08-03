struct Bounds(usize, usize);

fn main() {
    let b = Bounds(12, 2);
    assert_eq!(b.0, 12);
    assert_eq!(b.1, 2);

    let Bounds(x, y) = b;
    assert_eq!(x, 12);
    assert_eq!(y, 2);
}

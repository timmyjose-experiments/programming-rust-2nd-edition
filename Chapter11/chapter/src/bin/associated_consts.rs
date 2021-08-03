use std::ops::Add;

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

// Default could also work in this scenaior, but that restricts it to a single
// value for each T. Using associated constants, we can define as many required
// primitive base values as we wish.
fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}

fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

fn main() {
    assert_eq!(f32::ZERO, 0.0);
    assert_eq!(f64::ZERO, 0.0);
    assert_eq!(f32::ONE, 1.0);
    assert_eq!(f64::ONE, 1.0);

    assert_eq!(add_one::<f64>(99.0), 100.0);

    println!("{}", fib::<f64>(20));
}

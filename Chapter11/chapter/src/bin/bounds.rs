use std::default::Default;

use std::ops::{AddAssign, Mul};

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Copy + Default + AddAssign<N> + Mul<Output = N>,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }
    total
}

fn main() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

use std::ops::{Neg, Not};

//trait Neg {
//    type Output;
//
//    fn neg(self) -> Self::Output;
//}
//
//trait Not {
//    type Output;
//
//    fn not(self) -> Self::Output;
//}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T: Not<Output = T>> Not for Complex<T> {
    type Output = Complex<T>;

    fn not(self) -> Self::Output {
        Complex {
            re: !self.re,
            im: !self.im,
        }
    }
}

fn main() {
    let z1 = Complex::new(1, -2);
    assert_eq!(-z1, Complex::new(-1, 2));
    assert_eq!(!z1, Complex::new(-2, 1));
}

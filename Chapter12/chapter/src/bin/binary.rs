use std::cmp::{Eq, PartialEq};
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::{Neg, Not};

#[derive(Copy, Clone, Debug)]
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

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, rhs: &Complex<T>) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

impl<T: Eq> Eq for Complex<T> {}

fn main() {
    let z1 = Complex::new(1, 2);
    assert_eq!(-z1, Complex::new(-1, -2));

    let z2 = Complex::new(-2, -1);
    assert_eq!(!z2, Complex::new(1, 0));

    let z3 = z1 + z2;
    assert_eq!(z3, Complex::new(-1, 1));

    let mut z4 = Complex::new(1, 2);
    z4 += z4;
    assert_eq!(z4, Complex::new(2, 4));
}

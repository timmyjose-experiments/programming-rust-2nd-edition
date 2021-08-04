use std::fmt;
use std::ops::Add;

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

impl<T: fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}

//impl<T> Add for Complex<T>
//where
//    T: Add<Output = T>,
//{
//    type Output = Complex<T>;
//
//    fn add(self, rhs: Self) -> Self::Output {
//        Complex {
//            re: self.re + rhs.re,
//            im: self.im + rhs.im,
//        }
//    }
//}

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;

    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

fn main() {
    let z1 = Complex::new(1, 2);
    let z2 = Complex::new(-1, -2);

    let z3 = z1 + z2;
    println!("{} + {} = {}", z1, z2, z3);
}

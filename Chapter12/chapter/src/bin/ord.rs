use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq, Eq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, rhs: &Interval<T>) -> Option<Ordering> {
        if self == rhs {
            Some(Ordering::Equal)
        } else if self.lower >= rhs.upper {
            Some(Ordering::Greater)
        } else if self.upper <= rhs.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

fn main() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );

    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
}

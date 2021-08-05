use std::ops::{Deref, DerefMut};

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut s = Selector {
        elements: vec!["apple", "orange", "banana", "lychee", "strawberry"],
        current: 0,
    };

    assert_eq!(*s, "apple");
    *s = "pears";
    assert_eq!(*s, "pears");
}
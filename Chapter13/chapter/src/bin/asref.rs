use std::convert::{AsMut, AsRef};
use std::fmt::Debug;

fn hello<T: AsRef<str> + Debug>(val: T) {
    println!("{:?}", val);
}

fn add_one<T: AsMut<u64>>(val: &mut T) {
    *val.as_mut() += 1;
}

fn main() {
    hello("hello");
    hello("hello".to_string());

    let mut boxed_num = Box::new(99);
    add_one(&mut boxed_num);
    assert_eq!(*boxed_num, 100);
}
use std::iter::Iterator;

struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.state;
        self.state += 1;

        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let mut iter = Alternate { state: 0 };
    for _ in 0..5 {
        println!("{:?}", iter.next());
    }

    let mut iter = iter.fuse();

    for _ in 0..5 {
        println!("{:?}", iter.next());
    }
}

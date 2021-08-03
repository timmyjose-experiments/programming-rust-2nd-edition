struct Extrema<'elt> {
    least: &'elt i32,
    greatest: &'elt i32,
}

impl<'elt> Extrema<'elt> {
    pub fn new(least: &'elt i32, greatest: &'elt i32) -> Self {
        Extrema { least, greatest }
    }
}

fn find_extrema(slice: &[i32]) -> Extrema {
    assert!(slice.len() != 0);

    let mut least = &slice[0];
    let mut greatest = &slice[0];

    for elem in &slice[1..] {
        if *elem < *least {
            least = elem;
        }

        if *elem > *greatest {
            greatest = elem;
        }
    }

    Extrema::new(least, greatest)
}

fn main() {
    let Extrema { least, greatest } = find_extrema(&[1, 2, -1, -2, 0, 11, 99]);

    assert_eq!(*least, -2);
    assert_eq!(*greatest, 99);
}

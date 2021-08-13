use rand::random;
use std::fmt::Debug;
use std::iter::{from_fn, successors, IntoIterator};

fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

fn triangle(n: i32) -> i32 {
    let mut s = 0;
    for i in 1..=n {
        s += i;
    }
    s
}

fn triangle_again(n: i32) -> i32 {
    (1..=n).sum()
}

fn triangle_yet_again(n: i32) -> i32 {
    (1..=n).fold(0, |acc, m| acc + m)
}

fn from_fn_demo() {
    let mut count = 0;

    let counters = from_fn(|| {
        count += 1;

        if count > 10 {
            None
        } else {
            Some(count)
        }
    })
    .take(100)
    .collect::<Vec<_>>();
    println!("{:#?}", counters);
}

fn random_number_generator() -> impl Iterator<Item = f64> {
    from_fn(|| Some(random::<f64>()))
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn powers_of_10() -> impl Iterator<Item = usize> {
    successors(Some(1usize), |n| n.checked_mul(10usize))
}

fn main() {
    assert_eq!(triangle(10), 55);
    assert_eq!(triangle_again(10), 55);
    assert_eq!(triangle_yet_again(10), 55);

    dump(vec![1, 2, 3, 4, 5]);

    let lengths = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect::<Vec<_>>();
    println!("{:#.5?}", lengths);

    from_fn_demo();
    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );

    let mut randomizer = random_number_generator();
    for _ in 0..10 {
        println!("{:.5}", randomizer.next().unwrap());
    }

    let mut pow_iter = powers_of_10();
    for _ in 0..10 {
        println!("{:.5}", pow_iter.next().unwrap());
    }
}
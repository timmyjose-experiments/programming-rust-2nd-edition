#![feature(test)]

extern crate test;

use std::thread;

const NPROCESSORS: usize = 8;

fn factorial(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn process_factorial(chunk: Vec<u128>) -> Vec<u128> {
    chunk.iter().map(|&n| factorial(n)).collect::<Vec<_>>()
}

fn sequential_demo() -> Vec<u128> {
    let mut res = Vec::new();
    let input = (1..=24).collect::<Vec<u128>>();

    for chunk in input.chunks(5) {
        res.extend(process_factorial(chunk.to_vec()));
    }

    res
}

fn parallel_demo() -> Vec<u128> {
    let mut res = Vec::new();
    let input = (1..=24).collect::<Vec<u128>>();

    let mut handles = Vec::new();
    for chunk in input.chunks(NPROCESSORS) {
        let chunk_clone = chunk.to_owned();
        handles.push(thread::spawn(move || {
            process_factorial(chunk_clone.to_vec())
        }));
    }

    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn test_sequential(b: &mut Bencher) {
        b.iter(|| {
            let seq_res = black_box(sequential_demo());
            assert_eq!(
                seq_res,
                [
                    1,
                    2,
                    6,
                    24,
                    120,
                    720,
                    5040,
                    40320,
                    362880,
                    3628800,
                    39916800,
                    479001600,
                    6227020800,
                    87178291200,
                    1307674368000,
                    20922789888000,
                    355687428096000,
                    6402373705728000,
                    121645100408832000,
                    2432902008176640000,
                    51090942171709440000,
                    1124000727777607680000,
                    25852016738884976640000,
                    620448401733239439360000
                ]
            );
        });
    }

    #[bench]
    fn test_parallel(b: &mut Bencher) {
        b.iter(|| {
            let parallel_res = black_box(parallel_demo());
            assert_eq!(
                parallel_res,
                [
                    1,
                    2,
                    6,
                    24,
                    120,
                    720,
                    5040,
                    40320,
                    362880,
                    3628800,
                    39916800,
                    479001600,
                    6227020800,
                    87178291200,
                    1307674368000,
                    20922789888000,
                    355687428096000,
                    6402373705728000,
                    121645100408832000,
                    2432902008176640000,
                    51090942171709440000,
                    1124000727777607680000,
                    25852016738884976640000,
                    620448401733239439360000
                ]
            );
        });
    }
}
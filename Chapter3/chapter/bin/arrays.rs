#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

fn main() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let primes = sieve::<10000>();
    primes.iter().take(100).for_each(|p| print!("{} ", p));
    println!();
}

fn sieve<const N: usize>() -> Vec<u32>
where
    [(); N + 1]: ,
{
    let mut bs = [true; N + 1];

    for i in 2..=N {
        if bs[i] {
            let mut j = i * i;
            while j <= N {
                bs[j] = false;
                j += i;
            }
        }
    }

    let mut primes = Vec::new();
    for idx in 2..=N {
        if bs[idx] {
            primes.push(idx as u32);
        }
    }

    primes
}

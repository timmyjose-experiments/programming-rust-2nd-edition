pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);

    while n != 0 {
        let t = m % n;
        m = n;
        n = t;
    }
    m
}

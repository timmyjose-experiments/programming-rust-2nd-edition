pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);

    while n != 0 {
        let t = m % n;
        m = n;
        n = t;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn corner_cases() {
        assert_eq!(gcd(12, 1), 1);
        assert_eq!(gcd(1, 12), 1);
    }

    #[test]
    fn basic_gcd() {
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn complex_test() {
        assert_eq!(gcd(gcd(12, 18), gcd(12, 24)), 6);
    }
}

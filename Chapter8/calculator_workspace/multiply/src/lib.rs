pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(11, 2), 22);
    }
}

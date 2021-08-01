pub fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    use super::subtract;

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(11, 2), 9);
    }
}

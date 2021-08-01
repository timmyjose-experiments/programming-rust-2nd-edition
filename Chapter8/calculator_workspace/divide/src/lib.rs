pub fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else {
        x / y
    }
}

#[cfg(test)]
mod tests {
    use super::divide;

    #[test]
    fn test_divide() {
        assert_eq!(divide(11, 2), 5);
    }
}

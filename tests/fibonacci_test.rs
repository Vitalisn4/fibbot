#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;

    #[test]
    fn test_fibonacci() {
        // Test Fibonacci for standard values
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);

        // Test Fibonacci for large values
        assert_eq!(fibonacci(89), 1779979416004714189);
        assert_eq!(fibonacci(99), 218922995834555169026);
        assert_eq!(fibonacci(100), 354224848179261915075); // Test a large value
    }
}


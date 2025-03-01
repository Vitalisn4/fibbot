#[cfg(test)]
mod tests {
    use crate::extract_numbers::extract_numbers;

    #[test]
    fn test_extract_numbers() {
        // Test extracting numbers from text
        let text = "The numbers are 123, 456, and 789.".to_string();
        assert_eq!(extract_numbers(&text), vec![123, 456, 789]);

        let text = "No numbers here!".to_string();
        assert_eq!(extract_numbers(&text), Vec::<u32>::new());

        let text = "Some single numbers 0 and 99".to_string();
        assert_eq!(extract_numbers(&text), vec![0, 99]);

        let text = "Random content 42, 56, 78.".to_string();
        assert_eq!(extract_numbers(&text), vec![42, 56, 78]);

        // Edge case: Negative numbers
        let text = "Negative numbers: -123 and -456".to_string();
        assert_eq!(extract_numbers(&text), vec![-123, -456]);
    }
}

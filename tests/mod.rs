use regexy::utils::is_match;

/// Tests for the `is_match` function.
///
/// This module contains unit tests for the functionality of the `is_match` function.
///
/// To run these tests, navigate to your project directory and run:
///
/// ```bash
/// cargo test
/// ```
#[cfg(test)]
mod tests {
    use super::*; // This brings all items in the parent module into scope.
    #[test]
    fn test_is_match() {
        let pattern = r"\d+";
        let text = "There are 123 apples.";
        assert!(is_match(pattern, text));

        let text_no_match = "No digits here!";
        assert!(!is_match(pattern, text_no_match));

        // Additional tests
        let pattern_alpha = r"[a-zA-Z]+";
        let text_alpha = "Hello, world!";
        assert!(is_match(pattern_alpha, text_alpha));

        let text_no_alpha = "12345";
        assert!(!is_match(pattern_alpha, text_no_alpha));

        let pattern_email = r"^\S+@\S+\.\S+$";
        let text_email_valid = "example@example.com";
        let text_email_invalid = "example.com";
        assert!(is_match(pattern_email, text_email_valid));
        assert!(!is_match(pattern_email, text_email_invalid));
    }
}

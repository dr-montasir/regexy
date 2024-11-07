use regex::Regex;

/// Checks if the given text matches the specified regex pattern.
///
/// # Parameters
///
/// - `pattern`: A string slice that holds the regex pattern.
/// - `text`: A string slice that holds the text to be matched against the pattern.
///
/// # Returns
///
/// Returns `true` if the text matches the pattern, otherwise returns `false`.
///
/// # Example
///
/// ```
/// use regexy::utils::is_match;
///
/// let pattern = r"hello";
/// let text = "hello world!";
///
/// assert!(is_match(pattern, text));
/// ```
///
/// # Errors
///
/// This function will panic if the provided regex pattern is invalid.
///
/// # Panics
///
/// This function will panic if the regex compilation fails due to an invalid pattern.
///
/// <small>End Fun Doc</small>
pub fn is_match(pattern: &str, text: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}

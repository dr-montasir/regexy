#![doc(
    html_logo_url = "https://github.com/dr-montasir/regexy/raw/HEAD/logo.svg?sanitize=true",
    html_root_url = "https://docs.rs/regexy/latest/regexy"
)]

/// This library provides utility functions for working with regular expressions.
///
/// It provides core functions that allow users to check if a given text matches
/// a specified regex pattern using the `regex` crate.
///
/// ## Example
///
/// ```
/// use regexy::utils::is_match;
///
/// let pattern = r"\d+"; // Example pattern to match one or more digits
/// let text = "There are 123 apples.";
///
/// if is_match(pattern, text) {
///     println!("The text matches the pattern!");
/// } else {
///     println!("The text does not match the pattern.");
/// }
/// ```
///
/// # Regex Type
///
/// The `Regex` type, re-exported from the `regex` crate, provides efficient
/// functionality for compiling and using regular expressions. Users are encouraged
/// to utilize the `Regex` type directly for more complex regex operations,
/// such as regex capturing, iteration over matches, and replacement.
///
/// ## Example of Regex usage
///
/// ```
/// use regex::Regex;
///
/// let re = Regex::new(r"\w+").unwrap();
/// let text = "Hello, world!";
///
/// // Using Regex methods directly
/// for word in re.find_iter(text) {
///     println!("Found a word: {}", word.as_str());
/// }
/// ```
pub use regex::Regex; // Re-exported for public usage

/// Utilities module for regular expression-related functionalities.
///
/// The `utils` module contains utility functions for working with regular expressions.
/// It simplifies the matching process by providing higher-level functionality,
/// such as checking if a given text matches a regex pattern. The primary function
/// in this module is `is_match`, which facilitates easy regex matching without
/// needing to deal with the underlying regex compilation directly.
///
/// ## Examples
///
/// Here is how you can use the functions in the `utils` module:
///
/// ```
/// use regexy::utils::is_match;
///
/// let pattern = r"[a-zA-Z]+"; // Pattern to match one or more alphabetic characters
/// let text = "Hello, Rust!";
///
/// if is_match(pattern, text) {
///     println!("The text contains alphabetic characters.");
/// } else {
///     println!("No alphabetic characters found.");
/// }
/// ```
pub mod utils;

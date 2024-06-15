//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Calculates `left` + `right` and returns the result.
///
/// # Examples
///
/// ```
/// assert_eq!(template_rust::add(1, 2), 3);
/// assert_eq!(template_rust::add(5, 5), 10);
/// ```
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

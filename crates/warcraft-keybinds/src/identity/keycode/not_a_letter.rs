use std::fmt;

/// Returned when a character is not an ASCII letter A-Z.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NotALetter;

impl fmt::Display for NotALetter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not an ASCII letter A to Z")
    }
}

impl std::error::Error for NotALetter {}

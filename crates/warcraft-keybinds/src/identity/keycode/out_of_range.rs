use std::fmt;

/// Returned when a raw number is not a key Warcraft III accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct KeyCodeOutOfRange;

impl fmt::Display for KeyCodeOutOfRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not a valid Warcraft III keycode")
    }
}

impl std::error::Error for KeyCodeOutOfRange {}

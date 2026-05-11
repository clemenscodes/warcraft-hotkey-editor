use std::fmt;

use serde::{Deserialize, Serialize};

pub const SUPPORTED_VERSION_STRING: &str = "2.0.4.23745";

pub const SUPPORTED_VERSION: WarcraftVersion = WarcraftVersion::parse(SUPPORTED_VERSION_STRING);

/// Structured representation of a Warcraft III patch version:
/// `<major>.<minor>.<patch>.<build>`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WarcraftVersion {
    major: u32,
    minor: u32,
    patch: u32,
    build: u64,
}

impl WarcraftVersion {
    pub const fn new(major: u32, minor: u32, patch: u32, build: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }

    /// Parse a `"major.minor.patch.build"` string at compile time.
    /// Panics at const-eval time (i.e. at compile time) if the string
    /// doesn't have exactly three dots or contains a non-digit segment.
    pub const fn parse(dotted: &str) -> Self {
        let bytes = dotted.as_bytes();
        let length = bytes.len();

        let mut dot_positions: [usize; 3] = [0; 3];
        let mut dots_seen: usize = 0;
        let mut index: usize = 0;
        while index < length {
            if bytes[index] == b'.' {
                if dots_seen >= 3 {
                    panic!("WarcraftVersion::parse: version string has more than 3 dots");
                }
                dot_positions[dots_seen] = index;
                dots_seen += 1;
            }
            index += 1;
        }
        if dots_seen != 3 {
            panic!("WarcraftVersion::parse: version must be major.minor.patch.build");
        }

        let major = Self::parse_u32(bytes, 0, dot_positions[0]);
        let minor = Self::parse_u32(bytes, dot_positions[0] + 1, dot_positions[1]);
        let patch = Self::parse_u32(bytes, dot_positions[1] + 1, dot_positions[2]);
        let build = Self::parse_u64(bytes, dot_positions[2] + 1, length);
        Self::new(major, minor, patch, build)
    }

    const fn parse_u32(bytes: &[u8], start: usize, end: usize) -> u32 {
        if start >= end {
            panic!("WarcraftVersion::parse: empty version segment");
        }
        let mut result: u32 = 0;
        let mut cursor = start;
        while cursor < end {
            let byte = bytes[cursor];
            let digit: u32 = match byte {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => panic!("WarcraftVersion::parse: non-digit in version segment"),
            };
            result = result * 10 + digit;
            cursor += 1;
        }
        result
    }

    const fn parse_u64(bytes: &[u8], start: usize, end: usize) -> u64 {
        if start >= end {
            panic!("WarcraftVersion::parse: empty build segment");
        }
        let mut result: u64 = 0;
        let mut cursor = start;
        while cursor < end {
            let byte = bytes[cursor];
            let digit: u64 = match byte {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => panic!("WarcraftVersion::parse: non-digit in build segment"),
            };
            result = result * 10 + digit;
            cursor += 1;
        }
        result
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }

    pub fn build(&self) -> u64 {
        self.build
    }
}

impl fmt::Display for WarcraftVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.build
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_and_string_constants_agree() {
        assert_eq!(SUPPORTED_VERSION.to_string(), SUPPORTED_VERSION_STRING);
    }

    #[test]
    fn warcraft_version_equality_is_field_wise() {
        let version_a = WarcraftVersion::new(2, 0, 4, 23745);
        let version_b = WarcraftVersion::new(2, 0, 4, 23745);
        let version_c = WarcraftVersion::new(2, 0, 4, 23746);
        assert_eq!(version_a, version_b);
        assert_ne!(version_a, version_c);
    }

    #[test]
    fn parse_reconstructs_fields_from_dotted_string() {
        let parsed = WarcraftVersion::parse("2.0.4.23745");
        assert_eq!(parsed, WarcraftVersion::new(2, 0, 4, 23745));
    }

    #[test]
    fn parse_handles_larger_build_numbers() {
        let parsed = WarcraftVersion::parse("1.35.0.14481");
        assert_eq!(parsed.major(), 1);
        assert_eq!(parsed.minor(), 35);
        assert_eq!(parsed.patch(), 0);
        assert_eq!(parsed.build(), 14481);
    }
}

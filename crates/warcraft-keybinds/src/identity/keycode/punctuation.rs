use super::out_of_range::KeyCodeOutOfRange;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;

/// A punctuation key (the OEM keys on a US layout).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Punctuation {
    Semicolon,
    Equals,
    Comma,
    Minus,
    Period,
    Slash,
    Backtick,
    LeftBracket,
    Backslash,
    RightBracket,
    Quote,
}

impl Punctuation {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Semicolon => ";",
            Self::Equals => "=",
            Self::Comma => ",",
            Self::Minus => "-",
            Self::Period => ".",
            Self::Slash => "/",
            Self::Backtick => "`",
            Self::LeftBracket => "[",
            Self::Backslash => "\\",
            Self::RightBracket => "]",
            Self::Quote => "'",
        }
    }
}

impl TryFrom<u32> for Punctuation {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        match code {
            186 => Ok(Self::Semicolon),
            187 => Ok(Self::Equals),
            188 => Ok(Self::Comma),
            189 => Ok(Self::Minus),
            190 => Ok(Self::Period),
            191 => Ok(Self::Slash),
            192 => Ok(Self::Backtick),
            219 => Ok(Self::LeftBracket),
            220 => Ok(Self::Backslash),
            221 => Ok(Self::RightBracket),
            222 => Ok(Self::Quote),
            _ => Err(KeyCodeOutOfRange),
        }
    }
}

impl From<Punctuation> for u32 {
    fn from(punctuation: Punctuation) -> Self {
        match punctuation {
            Punctuation::Semicolon => 186,
            Punctuation::Equals => 187,
            Punctuation::Comma => 188,
            Punctuation::Minus => 189,
            Punctuation::Period => 190,
            Punctuation::Slash => 191,
            Punctuation::Backtick => 192,
            Punctuation::LeftBracket => 219,
            Punctuation::Backslash => 220,
            Punctuation::RightBracket => 221,
            Punctuation::Quote => 222,
        }
    }
}

impl Layered for Punctuation {
    type Layer = DomainLayer;
}

impl ValueObject for Punctuation {}

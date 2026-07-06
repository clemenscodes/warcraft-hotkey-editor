use super::out_of_range::KeyCodeOutOfRange;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;

/// A numeric-keypad key.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NumpadKey {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Multiply,
    Add,
    Subtract,
    Decimal,
    Divide,
}

impl NumpadKey {
    const DIGITS: [Self; 10] = [
        Self::Num0,
        Self::Num1,
        Self::Num2,
        Self::Num3,
        Self::Num4,
        Self::Num5,
        Self::Num6,
        Self::Num7,
        Self::Num8,
        Self::Num9,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Num0 => "Num0",
            Self::Num1 => "Num1",
            Self::Num2 => "Num2",
            Self::Num3 => "Num3",
            Self::Num4 => "Num4",
            Self::Num5 => "Num5",
            Self::Num6 => "Num6",
            Self::Num7 => "Num7",
            Self::Num8 => "Num8",
            Self::Num9 => "Num9",
            Self::Multiply => "Num*",
            Self::Add => "Num+",
            Self::Subtract => "Num-",
            Self::Decimal => "Num.",
            Self::Divide => "Num/",
        }
    }
}

impl TryFrom<u32> for NumpadKey {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        if (96..=105).contains(&code) {
            return Ok(Self::DIGITS[(code - 96) as usize]);
        }
        match code {
            106 => Ok(Self::Multiply),
            107 => Ok(Self::Add),
            109 => Ok(Self::Subtract),
            110 => Ok(Self::Decimal),
            111 => Ok(Self::Divide),
            _ => Err(KeyCodeOutOfRange),
        }
    }
}

impl From<NumpadKey> for u32 {
    fn from(key: NumpadKey) -> Self {
        match key {
            NumpadKey::Num0 => 96,
            NumpadKey::Num1 => 97,
            NumpadKey::Num2 => 98,
            NumpadKey::Num3 => 99,
            NumpadKey::Num4 => 100,
            NumpadKey::Num5 => 101,
            NumpadKey::Num6 => 102,
            NumpadKey::Num7 => 103,
            NumpadKey::Num8 => 104,
            NumpadKey::Num9 => 105,
            NumpadKey::Multiply => 106,
            NumpadKey::Add => 107,
            NumpadKey::Subtract => 109,
            NumpadKey::Decimal => 110,
            NumpadKey::Divide => 111,
        }
    }
}

impl Layered for NumpadKey {
    type Layer = DomainLayer;
}

impl ValueObject for NumpadKey {}

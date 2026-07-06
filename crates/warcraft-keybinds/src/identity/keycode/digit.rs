use super::out_of_range::KeyCodeOutOfRange;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;

/// A top-row digit key, 0 through 9.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    const ALL: [Self; 10] = [
        Self::Zero,
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
    ];

    /// The numeric value 0 through 9 this digit shows.
    pub(crate) fn value(self) -> u32 {
        let index = Self::ALL
            .iter()
            .position(|digit| *digit == self)
            .unwrap_or(0);
        u32::try_from(index).unwrap_or(0)
    }
}

impl TryFrom<u32> for Digit {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        if (48..=57).contains(&code) {
            return Ok(Self::ALL[(code - 48) as usize]);
        }
        Err(KeyCodeOutOfRange)
    }
}

impl From<Digit> for u32 {
    fn from(digit: Digit) -> Self {
        let index = Digit::ALL
            .iter()
            .position(|item| *item == digit)
            .unwrap_or(0);
        index as u32 + 48
    }
}

impl Layered for Digit {
    type Layer = DomainLayer;
}

impl ValueObject for Digit {}

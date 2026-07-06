use super::not_a_letter::NotALetter;
use super::out_of_range::KeyCodeOutOfRange;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;
use std::fmt;

/// A letter key, A through Z.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Letter {
    const ALL: [Self; 26] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
    ];

    /// The uppercase character this letter represents.
    pub fn character(self) -> char {
        let index = Self::ALL
            .iter()
            .position(|letter| *letter == self)
            .unwrap_or(0);
        let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        char::from(alphabet[index])
    }
}

impl TryFrom<u32> for Letter {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        if (65..=90).contains(&code) {
            return Ok(Self::ALL[(code - 65) as usize]);
        }
        Err(KeyCodeOutOfRange)
    }
}

impl From<Letter> for u32 {
    fn from(letter: Letter) -> Self {
        let index = Letter::ALL
            .iter()
            .position(|item| *item == letter)
            .unwrap_or(0);
        index as u32 + 65
    }
}

impl TryFrom<char> for Letter {
    type Error = NotALetter;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if !character.is_ascii_alphabetic() {
            return Err(NotALetter);
        }
        let upper = character.to_ascii_uppercase();
        Self::try_from(u32::from(upper)).map_err(|_| NotALetter)
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.character())
    }
}

impl Layered for Letter {
    type Layer = DomainLayer;
}

impl ValueObject for Letter {}

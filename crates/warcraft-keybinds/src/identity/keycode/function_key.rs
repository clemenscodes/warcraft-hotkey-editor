use super::out_of_range::KeyCodeOutOfRange;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;

/// A function key, F1 through F12.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FunctionKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

impl FunctionKey {
    const ALL: [Self; 12] = [
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
    ];

    /// The function key number 1 through 12.
    pub(crate) fn number(self) -> u32 {
        let index = Self::ALL.iter().position(|key| *key == self).unwrap_or(0);
        u32::try_from(index).unwrap_or(0) + 1
    }
}

impl TryFrom<u32> for FunctionKey {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        if (112..=123).contains(&code) {
            return Ok(Self::ALL[(code - 112) as usize]);
        }
        Err(KeyCodeOutOfRange)
    }
}

impl From<FunctionKey> for u32 {
    fn from(key: FunctionKey) -> Self {
        let index = FunctionKey::ALL
            .iter()
            .position(|item| *item == key)
            .unwrap_or(0);
        index as u32 + 112
    }
}

impl Layered for FunctionKey {
    type Layer = DomainLayer;
}

impl ValueObject for FunctionKey {}

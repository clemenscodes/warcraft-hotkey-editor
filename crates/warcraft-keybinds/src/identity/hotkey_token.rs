use crate::identity::keycode::{KeyCode, Letter, MouseButton};
use crate::model::Hotkey;
use std::fmt;

const ESCAPE_VK: u32 = 27;
const MOUSE_BACK_VK: u32 = 5;
const MOUSE_FORWARD_VK: u32 = 6;

/// A key bindable to an ability hotkey. Far narrower than a full keyboard: only
/// a letter, Escape, or a mouse side button. The letter is the precise [`Letter`]
/// enum, so a non-letter character cannot be represented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HotkeyToken {
    Letter(Letter),
    Escape,
    MouseForward,
    MouseBack,
}

impl HotkeyToken {
    pub fn display_label(self) -> String {
        match self {
            Self::Letter(letter) => letter.character().to_string(),
            Self::Escape => String::from("Esc"),
            Self::MouseForward => String::from("Mouse5"),
            Self::MouseBack => String::from("Mouse4"),
        }
    }
}

impl From<Letter> for HotkeyToken {
    fn from(letter: Letter) -> Self {
        Self::Letter(letter)
    }
}

impl TryFrom<char> for HotkeyToken {
    type Error = HotkeyTokenParseError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        let letter = Letter::try_from(character).map_err(|_| HotkeyTokenParseError)?;
        Ok(Self::Letter(letter))
    }
}

impl TryFrom<HotkeyToken> for char {
    type Error = HotkeyTokenIsNotLetter;

    fn try_from(token: HotkeyToken) -> Result<Self, Self::Error> {
        match token {
            HotkeyToken::Letter(letter) => Ok(letter.character()),
            _ => Err(HotkeyTokenIsNotLetter),
        }
    }
}

impl From<HotkeyToken> for KeyCode {
    fn from(token: HotkeyToken) -> Self {
        match token {
            HotkeyToken::Letter(letter) => Self::Letter(letter),
            HotkeyToken::Escape => Self::Escape,
            HotkeyToken::MouseForward => Self::Mouse(MouseButton::Forward),
            HotkeyToken::MouseBack => Self::Mouse(MouseButton::Back),
        }
    }
}

impl TryFrom<KeyCode> for HotkeyToken {
    type Error = KeyCodeIsNotHotkeyToken;

    fn try_from(key: KeyCode) -> Result<Self, Self::Error> {
        match key {
            KeyCode::Letter(letter) => Ok(Self::Letter(letter)),
            KeyCode::Escape => Ok(Self::Escape),
            KeyCode::Mouse(MouseButton::Forward) => Ok(Self::MouseForward),
            KeyCode::Mouse(MouseButton::Back) => Ok(Self::MouseBack),
            _ => Err(KeyCodeIsNotHotkeyToken),
        }
    }
}

impl fmt::Display for HotkeyToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.display_label();
        write!(formatter, "{label}")
    }
}

impl From<HotkeyToken> for Hotkey {
    fn from(token: HotkeyToken) -> Self {
        match token {
            HotkeyToken::Letter(letter) => Self::Letter(letter.character()),
            HotkeyToken::Escape => Self::VirtualKey(ESCAPE_VK),
            HotkeyToken::MouseForward => Self::VirtualKey(MOUSE_FORWARD_VK),
            HotkeyToken::MouseBack => Self::VirtualKey(MOUSE_BACK_VK),
        }
    }
}

impl TryFrom<&Hotkey> for HotkeyToken {
    type Error = ();

    fn try_from(hotkey: &Hotkey) -> Result<Self, ()> {
        match hotkey {
            Hotkey::Letter(character) => {
                let letter = Letter::try_from(*character).map_err(|_| ())?;
                Ok(Self::Letter(letter))
            }
            Hotkey::VirtualKey(ESCAPE_VK) => Ok(Self::Escape),
            Hotkey::VirtualKey(MOUSE_BACK_VK) => Ok(Self::MouseBack),
            Hotkey::VirtualKey(MOUSE_FORWARD_VK) => Ok(Self::MouseForward),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HotkeyTokenIsNotLetter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HotkeyTokenParseError;

/// A [`KeyCode`] that is not one of the keys an ability hotkey may bind: it is
/// neither a letter, Escape, nor a mouse side button, so it has no [`HotkeyToken`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct KeyCodeIsNotHotkeyToken;

impl TryFrom<&str> for HotkeyToken {
    type Error = HotkeyTokenParseError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(HotkeyTokenParseError);
        }
        let mut characters = trimmed.chars();
        let first_character = characters.next().ok_or(HotkeyTokenParseError)?;
        let is_single_character = characters.next().is_none();
        if is_single_character && first_character.is_ascii_alphabetic() {
            return Self::try_from(first_character);
        }
        if let Ok(code) = trimmed.parse::<u32>() {
            return match code {
                ESCAPE_VK => Ok(Self::Escape),
                MOUSE_FORWARD_VK => Ok(Self::MouseForward),
                MOUSE_BACK_VK => Ok(Self::MouseBack),
                _ => Err(HotkeyTokenParseError),
            };
        }
        Err(HotkeyTokenParseError)
    }
}

impl ddd::Layered for HotkeyToken {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for HotkeyToken {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::HotkeyToken;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn hotkey_token_is_a_value_object() {
        assert_value_object::<HotkeyToken>();
    }
}

#[cfg(test)]
mod keycode_conversion_tests {
    use super::{HotkeyToken, KeyCodeIsNotHotkeyToken};
    use crate::identity::keycode::{KeyCode, Letter};

    #[test]
    fn every_token_round_trips_through_its_keycode() {
        let letter = Letter::try_from('Q').expect("Q is a letter");
        let tokens = [
            HotkeyToken::Letter(letter),
            HotkeyToken::Escape,
            HotkeyToken::MouseForward,
            HotkeyToken::MouseBack,
        ];
        for token in tokens {
            let key = KeyCode::from(token);
            let recovered = HotkeyToken::try_from(key).expect("a token keycode is a token");
            assert_eq!(recovered, token);
        }
    }

    #[test]
    fn a_keycode_outside_the_token_set_has_no_token() {
        let key = KeyCode::Space;
        let recovered = HotkeyToken::try_from(key);
        assert_eq!(recovered, Err(KeyCodeIsNotHotkeyToken));
    }
}

use std::fmt;

use crate::identity::keycode::Letter;
use crate::model::Hotkey;

// CustomKeys.txt uses numeric Windows Virtual-Key codes for keys that aren't
// plain letters: VK_ESCAPE (27), VK_XBUTTON1 (5, "Mouse4"/back), and
// VK_XBUTTON2 (6, "Mouse5"/forward). Named tokens like "Escape" are not recognized.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HotkeyTokenIsNotLetter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HotkeyTokenParseError;

impl TryFrom<&str> for HotkeyToken {
    type Error = HotkeyTokenParseError;

    fn try_from(raw_value: &str) -> Result<Self, Self::Error> {
        let trimmed_value = raw_value.trim();
        if trimmed_value.is_empty() {
            return Err(HotkeyTokenParseError);
        }
        let mut characters = trimmed_value.chars();
        let first_character = characters.next().ok_or(HotkeyTokenParseError)?;
        let is_single_character = characters.next().is_none();
        if is_single_character && first_character.is_ascii_alphabetic() {
            return HotkeyToken::try_from(first_character);
        }
        if let Ok(code) = trimmed_value.parse::<u32>() {
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

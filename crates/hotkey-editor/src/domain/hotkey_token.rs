use std::fmt;

// CustomKeys.txt uses numeric Windows Virtual-Key codes for keys that aren't
// plain letters: VK_ESCAPE (27), VK_XBUTTON1 (5, "Mouse4"/back), and
// VK_XBUTTON2 (6, "Mouse5"/forward). The in-game hotkey editor writes these
// as `Hotkey=27` etc., and named tokens like "Escape" are not recognized.
const ESCAPE_TOKEN: &str = "27";
const MOUSE_BACK_TOKEN: &str = "5";
const MOUSE_FORWARD_TOKEN: &str = "6";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum HotkeyToken {
    Letter { character: char },
    Escape,
    MouseForward,
    MouseBack,
}

impl HotkeyToken {
    pub(crate) fn display_label(self) -> String {
        match self {
            Self::Letter { character } => character.to_string(),
            Self::Escape => String::from("Esc"),
            Self::MouseForward => String::from("Mouse5"),
            Self::MouseBack => String::from("Mouse4"),
        }
    }
}

impl From<char> for HotkeyToken {
    fn from(character: char) -> Self {
        let upper_character = character.to_ascii_uppercase();
        Self::Letter {
            character: upper_character,
        }
    }
}

impl TryFrom<HotkeyToken> for char {
    type Error = HotkeyTokenIsNotLetter;

    fn try_from(token: HotkeyToken) -> Result<Self, Self::Error> {
        match token {
            HotkeyToken::Letter { character } => Ok(character),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CustomKeysValue {
    raw: String,
}

impl CustomKeysValue {
    pub(crate) fn as_str(&self) -> &str {
        &self.raw
    }
}

impl From<HotkeyToken> for CustomKeysValue {
    fn from(token: HotkeyToken) -> Self {
        let raw = match token {
            HotkeyToken::Letter { character } => character.to_string(),
            HotkeyToken::Escape => String::from(ESCAPE_TOKEN),
            HotkeyToken::MouseForward => String::from(MOUSE_FORWARD_TOKEN),
            HotkeyToken::MouseBack => String::from(MOUSE_BACK_TOKEN),
        };
        Self { raw }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HotkeyTokenIsNotLetter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HotkeyTokenParseError;

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
            return Ok(HotkeyToken::from(first_character));
        }
        if trimmed_value.eq_ignore_ascii_case(ESCAPE_TOKEN) {
            return Ok(Self::Escape);
        }
        if trimmed_value.eq_ignore_ascii_case(MOUSE_FORWARD_TOKEN) {
            return Ok(Self::MouseForward);
        }
        if trimmed_value.eq_ignore_ascii_case(MOUSE_BACK_TOKEN) {
            return Ok(Self::MouseBack);
        }
        Err(HotkeyTokenParseError)
    }
}

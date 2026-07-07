use crate::identity::hotkey_token::HotkeyToken;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hotkey {
    Letter(char),
    FunctionKey(u8),
    VirtualKey(u32),
    MultiLevel { tokens: [Option<HotkeyToken>; 4] },
}

impl fmt::Display for Hotkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(character) => write!(formatter, "{character}"),
            Self::FunctionKey(number) => write!(formatter, "F{number}"),
            Self::VirtualKey(code) => write!(formatter, "{code}"),
            Self::MultiLevel { tokens } => {
                let mut first = true;
                for token in tokens.iter().flatten() {
                    if !first {
                        formatter.write_str(",")?;
                    }
                    write!(formatter, "{token}")?;
                    first = false;
                }
                Ok(())
            }
        }
    }
}

impl TryFrom<&str> for Hotkey {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, ()> {
        if text.is_empty() {
            return Err(());
        }
        if text.contains(',') {
            let mut tokens: [Option<HotkeyToken>; 4] = [None; 4];
            for (index, segment) in text.split(',').enumerate() {
                if index >= 4 {
                    return Err(());
                }
                tokens[index] = Some(HotkeyToken::try_from(segment.trim()).map_err(|_| ())?);
            }
            return Ok(Self::MultiLevel { tokens });
        }
        let lowercase = text.to_ascii_lowercase();
        if let Some(rest) = lowercase.strip_prefix('f')
            && let Ok(number) = rest.parse::<u8>()
            && (1..=12).contains(&number)
        {
            return Ok(Self::FunctionKey(number));
        }
        let mut character_iter = text.chars();
        if let Some(character) = character_iter.next()
            && character_iter.next().is_none()
            && character.is_ascii_alphabetic()
        {
            return Ok(Self::Letter(character.to_ascii_uppercase()));
        }
        if let Ok(code) = text.parse::<u32>() {
            return Ok(Self::VirtualKey(code));
        }
        Err(())
    }
}

impl From<char> for Hotkey {
    fn from(character: char) -> Self {
        Self::Letter(character.to_ascii_uppercase())
    }
}

impl From<Hotkey> for String {
    fn from(hotkey: Hotkey) -> Self {
        hotkey.to_string()
    }
}

impl Hotkey {
    pub fn first_token(&self) -> Option<HotkeyToken> {
        match self {
            Self::MultiLevel { tokens } => tokens.iter().flatten().next().copied(),
            other => HotkeyToken::try_from(other).ok(),
        }
    }

    pub fn level_count(&self) -> usize {
        match self {
            Self::MultiLevel { tokens } => tokens.iter().flatten().count(),
            _ => 1,
        }
    }

    pub fn replicated(token: HotkeyToken, count: usize) -> Self {
        let clamped_count = count.clamp(1, 4);
        if clamped_count == 1 {
            Self::from(token)
        } else {
            let mut tokens: [Option<HotkeyToken>; 4] = [None; 4];
            for slot in tokens.iter_mut().take(clamped_count) {
                *slot = Some(token);
            }
            Self::MultiLevel { tokens }
        }
    }

    pub fn accepts_grid_letter(&self) -> bool {
        let Some(token) = self.first_token() else {
            return true;
        };
        char::try_from(token).is_ok()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ParseHotkeyError;

impl fmt::Display for ParseHotkeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid hotkey")
    }
}

impl std::error::Error for ParseHotkeyError {}

impl FromStr for Hotkey {
    type Err = ParseHotkeyError;

    fn from_str(text: &str) -> Result<Self, ParseHotkeyError> {
        Self::try_from(text).map_err(|()| ParseHotkeyError)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityModifier {
    Alt,
    Ctrl,
    CtrlOrAlt,
    Shift,
}

impl fmt::Display for AbilityModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Alt => "Alt",
            Self::Ctrl => "Ctrl",
            Self::CtrlOrAlt => "Ctrl_or_Alt",
            Self::Shift => "Shift",
        };
        formatter.write_str(text)
    }
}

impl TryFrom<&str> for AbilityModifier {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, ()> {
        match text.to_ascii_lowercase().as_str() {
            "alt" => Ok(Self::Alt),
            "ctrl" => Ok(Self::Ctrl),
            "ctrl_or_alt" => Ok(Self::CtrlOrAlt),
            "shift" => Ok(Self::Shift),
            _ => Err(()),
        }
    }
}

impl From<AbilityModifier> for String {
    fn from(modifier: AbilityModifier) -> Self {
        modifier.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ParseAbilityModifierError;

impl fmt::Display for ParseAbilityModifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid ability modifier")
    }
}

impl std::error::Error for ParseAbilityModifierError {}

impl FromStr for AbilityModifier {
    type Err = ParseAbilityModifierError;

    fn from_str(text: &str) -> Result<Self, ParseAbilityModifierError> {
        Self::try_from(text).map_err(|()| ParseAbilityModifierError)
    }
}

impl ddd::Layered for Hotkey {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for Hotkey {}

impl ddd::Layered for AbilityModifier {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for AbilityModifier {}

#[cfg(test)]
mod hotkey_marker_tests {
    use super::AbilityModifier;
    use super::Hotkey;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn hotkey_and_modifier_are_value_objects() {
        assert_value_object::<Hotkey>();
        assert_value_object::<AbilityModifier>();
    }
}

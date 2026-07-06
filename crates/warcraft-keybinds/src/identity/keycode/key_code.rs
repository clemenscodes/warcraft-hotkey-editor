use super::digit::Digit;
use super::function_key::FunctionKey;
use super::letter::Letter;
use super::mouse_button::MouseButton;
use super::numpad_key::NumpadKey;
use super::out_of_range::KeyCodeOutOfRange;
use super::punctuation::Punctuation;
use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;
use std::fmt;

/// A key Warcraft III accepts in `CustomKeys.txt`. Every value is valid; there is
/// no open integer payload that could hold a nonexistent key.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum KeyCode {
    Letter(Letter),
    Digit(Digit),
    Function(FunctionKey),
    Numpad(NumpadKey),
    Punctuation(Punctuation),
    Mouse(MouseButton),
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    Pause,
    CapsLock,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    NumLock,
    ScrollLock,
}

impl TryFrom<u32> for KeyCode {
    type Error = KeyCodeOutOfRange;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        if let Ok(letter) = Letter::try_from(code) {
            return Ok(Self::Letter(letter));
        }
        if let Ok(digit) = Digit::try_from(code) {
            return Ok(Self::Digit(digit));
        }
        if let Ok(function_key) = FunctionKey::try_from(code) {
            return Ok(Self::Function(function_key));
        }
        if let Ok(numpad_key) = NumpadKey::try_from(code) {
            return Ok(Self::Numpad(numpad_key));
        }
        if let Ok(punctuation) = Punctuation::try_from(code) {
            return Ok(Self::Punctuation(punctuation));
        }
        let named = match code {
            5 => Self::Mouse(MouseButton::Back),
            6 => Self::Mouse(MouseButton::Forward),
            8 => Self::Backspace,
            9 => Self::Tab,
            13 => Self::Enter,
            16 => Self::Shift,
            17 => Self::Ctrl,
            18 => Self::Alt,
            19 => Self::Pause,
            20 => Self::CapsLock,
            27 => Self::Escape,
            32 => Self::Space,
            33 => Self::PageUp,
            34 => Self::PageDown,
            35 => Self::End,
            36 => Self::Home,
            37 => Self::Left,
            38 => Self::Up,
            39 => Self::Right,
            40 => Self::Down,
            45 => Self::Insert,
            46 => Self::Delete,
            144 => Self::NumLock,
            145 => Self::ScrollLock,
            _ => return Err(KeyCodeOutOfRange),
        };
        Ok(named)
    }
}

impl From<KeyCode> for u32 {
    fn from(key: KeyCode) -> Self {
        match key {
            KeyCode::Letter(letter) => u32::from(letter),
            KeyCode::Digit(digit) => u32::from(digit),
            KeyCode::Function(function_key) => u32::from(function_key),
            KeyCode::Numpad(numpad_key) => u32::from(numpad_key),
            KeyCode::Punctuation(punctuation) => u32::from(punctuation),
            KeyCode::Mouse(MouseButton::Back) => 5,
            KeyCode::Mouse(MouseButton::Forward) => 6,
            KeyCode::Backspace => 8,
            KeyCode::Tab => 9,
            KeyCode::Enter => 13,
            KeyCode::Shift => 16,
            KeyCode::Ctrl => 17,
            KeyCode::Alt => 18,
            KeyCode::Pause => 19,
            KeyCode::CapsLock => 20,
            KeyCode::Escape => 27,
            KeyCode::Space => 32,
            KeyCode::PageUp => 33,
            KeyCode::PageDown => 34,
            KeyCode::End => 35,
            KeyCode::Home => 36,
            KeyCode::Left => 37,
            KeyCode::Up => 38,
            KeyCode::Right => 39,
            KeyCode::Down => 40,
            KeyCode::Insert => 45,
            KeyCode::Delete => 46,
            KeyCode::NumLock => 144,
            KeyCode::ScrollLock => 145,
        }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(letter) => write!(formatter, "{}", letter.character()),
            Self::Digit(digit) => write!(formatter, "{}", digit.value()),
            Self::Function(function_key) => {
                write!(formatter, "F{}", function_key.number())
            }
            Self::Numpad(numpad_key) => formatter.write_str(numpad_key.label()),
            Self::Punctuation(punctuation) => formatter.write_str(punctuation.label()),
            Self::Mouse(MouseButton::Back) => formatter.write_str("Mouse4"),
            Self::Mouse(MouseButton::Forward) => formatter.write_str("Mouse5"),
            Self::Backspace => formatter.write_str("Backspace"),
            Self::Tab => formatter.write_str("Tab"),
            Self::Enter => formatter.write_str("Enter"),
            Self::Shift => formatter.write_str("Shift"),
            Self::Ctrl => formatter.write_str("Ctrl"),
            Self::Alt => formatter.write_str("Alt"),
            Self::Pause => formatter.write_str("Pause"),
            Self::CapsLock => formatter.write_str("CapsLock"),
            Self::Escape => formatter.write_str("Esc"),
            Self::Space => formatter.write_str("Space"),
            Self::PageUp => formatter.write_str("PageUp"),
            Self::PageDown => formatter.write_str("PageDown"),
            Self::End => formatter.write_str("End"),
            Self::Home => formatter.write_str("Home"),
            Self::Left => formatter.write_str("Left"),
            Self::Up => formatter.write_str("Up"),
            Self::Right => formatter.write_str("Right"),
            Self::Down => formatter.write_str("Down"),
            Self::Insert => formatter.write_str("Insert"),
            Self::Delete => formatter.write_str("Delete"),
            Self::NumLock => formatter.write_str("NumLock"),
            Self::ScrollLock => formatter.write_str("ScrollLock"),
        }
    }
}

impl Layered for KeyCode {
    type Layer = DomainLayer;
}

impl ValueObject for KeyCode {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn rejects_codes_that_are_not_real_keys() {
        assert_eq!(KeyCode::try_from(0), Err(KeyCodeOutOfRange));
        assert_eq!(KeyCode::try_from(255), Err(KeyCodeOutOfRange));
        assert_eq!(KeyCode::try_from(999), Err(KeyCodeOutOfRange));
        assert_eq!(KeyCode::try_from(108), Err(KeyCodeOutOfRange));
    }

    #[test]
    fn every_valid_code_round_trips_through_u32() {
        let valid_codes = [
            5, 6, 8, 9, 13, 16, 17, 18, 19, 20, 27, 32, 33, 34, 35, 36, 37, 38, 39, 40, 45, 46, 48,
            49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 77, 90, 96, 97, 105, 106, 107, 109, 110, 111,
            112, 117, 123, 144, 145, 186, 187, 188, 189, 190, 191, 192, 219, 220, 221, 222,
        ];
        for code in valid_codes {
            let key = KeyCode::try_from(code).expect("valid code");
            assert_eq!(u32::from(key), code, "round trip failed for {code}");
        }
    }

    #[test]
    fn displays_human_labels() {
        assert_eq!(KeyCode::try_from(65).unwrap().to_string(), "A");
        assert_eq!(KeyCode::try_from(49).unwrap().to_string(), "1");
        assert_eq!(KeyCode::try_from(117).unwrap().to_string(), "F6");
        assert_eq!(KeyCode::try_from(96).unwrap().to_string(), "Num0");
        assert_eq!(KeyCode::try_from(27).unwrap().to_string(), "Esc");
        assert_eq!(KeyCode::try_from(5).unwrap().to_string(), "Mouse4");
    }

    #[test]
    fn keycode_family_are_value_objects() {
        assert_value_object::<KeyCode>();
        assert_value_object::<Letter>();
        assert_value_object::<Digit>();
        assert_value_object::<FunctionKey>();
        assert_value_object::<NumpadKey>();
        assert_value_object::<Punctuation>();
        assert_value_object::<MouseButton>();
    }
}

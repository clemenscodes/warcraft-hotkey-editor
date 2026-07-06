//! A precise keyboard key. Every value of [`KeyCode`] is a key Warcraft III
//! actually accepts in `CustomKeys.txt`; invalid codes cannot be represented. The
//! only way in from a raw number is [`TryFrom<u32>`], which rejects anything that
//! is not a real key, so `999` or `255` never become a `KeyCode`.

use std::fmt;

/// Returned when a raw number is not a key Warcraft III accepts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeyCodeOutOfRange;

impl fmt::Display for KeyCodeOutOfRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not a valid Warcraft III keycode")
    }
}

impl std::error::Error for KeyCodeOutOfRange {}

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

/// Returned when a character is not an ASCII letter A-Z.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NotALetter;

impl fmt::Display for NotALetter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not an ASCII letter A to Z")
    }
}

impl std::error::Error for NotALetter {}

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
    fn value(self) -> u32 {
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
    fn number(self) -> u32 {
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

    fn label(self) -> &'static str {
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
    fn label(self) -> &'static str {
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

/// A mouse side button Warcraft III can bind.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseButton {
    Back,
    Forward,
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

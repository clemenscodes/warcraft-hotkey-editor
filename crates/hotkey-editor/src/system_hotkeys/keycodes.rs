use std::fmt;

use warcraft_api::SystemKeybindModifier;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct KeyCode {
    value: u32,
}

impl From<u32> for KeyCode {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            5 => formatter.write_str("Mouse4"),
            6 => formatter.write_str("Mouse5"),
            8 => formatter.write_str("Backspace"),
            9 => formatter.write_str("Tab"),
            13 => formatter.write_str("Enter"),
            16 => formatter.write_str("Shift"),
            17 => formatter.write_str("Ctrl"),
            18 => formatter.write_str("Alt"),
            19 => formatter.write_str("Pause"),
            20 => formatter.write_str("CapsLock"),
            27 => formatter.write_str("Esc"),
            32 => formatter.write_str("Space"),
            33 => formatter.write_str("PageUp"),
            34 => formatter.write_str("PageDown"),
            35 => formatter.write_str("End"),
            36 => formatter.write_str("Home"),
            37 => formatter.write_str("Left"),
            38 => formatter.write_str("Up"),
            39 => formatter.write_str("Right"),
            40 => formatter.write_str("Down"),
            45 => formatter.write_str("Insert"),
            46 => formatter.write_str("Delete"),
            48..=57 => {
                let digit = self.value - 48;
                write!(formatter, "{digit}")
            }
            65..=90 => {
                let key_character = char::from_u32(self.value).unwrap_or('?');
                write!(formatter, "{key_character}")
            }
            96..=105 => {
                let numpad_digit = self.value - 96;
                write!(formatter, "Num{numpad_digit}")
            }
            106 => formatter.write_str("Num*"),
            107 => formatter.write_str("Num+"),
            109 => formatter.write_str("Num-"),
            110 => formatter.write_str("Num."),
            111 => formatter.write_str("Num/"),
            112..=123 => {
                let function_index = self.value - 111;
                write!(formatter, "F{function_index}")
            }
            144 => formatter.write_str("NumLock"),
            145 => formatter.write_str("ScrollLock"),
            186 => formatter.write_str(";"),
            187 => formatter.write_str("="),
            188 => formatter.write_str(","),
            189 => formatter.write_str("-"),
            190 => formatter.write_str("."),
            191 => formatter.write_str("/"),
            192 => formatter.write_str("`"),
            219 => formatter.write_str("["),
            220 => formatter.write_str("\\"),
            221 => formatter.write_str("]"),
            222 => formatter.write_str("'"),
            other_code => write!(formatter, "Key {other_code}"),
        }
    }
}

pub(crate) struct KeyCodes;

impl KeyCodes {
    pub(crate) fn from_event(event_key: &str, event_code: &str) -> Option<u32> {
        if event_key.len() == 1 {
            let first_character = event_key.chars().next()?;
            if first_character.is_ascii_alphabetic() {
                let upper_byte =
                    u8::try_from(u32::from(first_character.to_ascii_uppercase())).ok()?;
                return Some(u32::from(upper_byte));
            }
            if first_character.is_ascii_digit() {
                let digit_byte = u8::try_from(u32::from(first_character)).ok()?;
                return Some(u32::from(digit_byte));
            }
            match first_character {
                ' ' => return Some(32),
                '`' => return Some(192),
                '-' => return Some(189),
                '=' => return Some(187),
                '[' => return Some(219),
                ']' => return Some(221),
                '\\' => return Some(220),
                ';' => return Some(186),
                '\'' => return Some(222),
                ',' => return Some(188),
                '.' => return Some(190),
                '/' => return Some(191),
                _ => {}
            }
        }
        match event_key {
            "Tab" => Some(9),
            "Backspace" => Some(8),
            "Enter" => Some(13),
            "Escape" => Some(27),
            "Insert" => Some(45),
            "Delete" => Some(46),
            "Home" => Some(36),
            "End" => Some(35),
            "PageUp" => Some(33),
            "PageDown" => Some(34),
            "ArrowLeft" => Some(37),
            "ArrowUp" => Some(38),
            "ArrowRight" => Some(39),
            "ArrowDown" => Some(40),
            " " => Some(32),
            _ => {
                if let Some(rest) = event_key.strip_prefix('F')
                    && let Ok(number) = rest.parse::<u32>()
                    && (1..=12).contains(&number)
                {
                    return Some(111 + number);
                }
                if let Some(suffix) = event_code.strip_prefix("Numpad")
                    && let Ok(number) = suffix.parse::<u32>()
                    && number <= 9
                {
                    return Some(96 + number);
                }
                None
            }
        }
    }

    pub(crate) fn modifier_prefix(modifier: SystemKeybindModifier) -> &'static str {
        match modifier {
            SystemKeybindModifier::None => "",
            SystemKeybindModifier::Alt => "Alt + ",
            SystemKeybindModifier::Ctrl => "Ctrl + ",
            SystemKeybindModifier::CtrlOrAlt => "Ctrl/Alt + ",
            SystemKeybindModifier::Shift => "Shift + ",
        }
    }
}

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
}

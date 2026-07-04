use warcraft_keybinds::KeyCode;

/// A browser `KeyboardEvent`, narrowed to the two fields the picker reads. This
/// is the only place that knows the shape of a web key event; it translates one
/// into the domain's [`KeyCode`] and makes no decision beyond that mapping.
pub(super) struct BrowserKeyEvent<'a> {
    key: &'a str,
    code: &'a str,
}

impl<'a> BrowserKeyEvent<'a> {
    pub(super) fn new(key: &'a str, code: &'a str) -> Self {
        Self { key, code }
    }

    /// The domain key this event names, or `None` when the browser reported a
    /// key Warcraft III does not accept.
    pub(super) fn key_code(&self) -> Option<KeyCode> {
        let raw_code = self.raw_code()?;
        KeyCode::try_from(raw_code).ok()
    }

    fn raw_code(&self) -> Option<u32> {
        if self.key.len() == 1 {
            let first_character = self.key.chars().next()?;
            if first_character.is_ascii_alphabetic() {
                let upper_character = first_character.to_ascii_uppercase();
                return Some(u32::from(upper_character));
            }
            if first_character.is_ascii_digit() {
                return Some(u32::from(first_character));
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
        match self.key {
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
                if let Some(rest) = self.key.strip_prefix('F')
                    && let Ok(number) = rest.parse::<u32>()
                    && (1..=12).contains(&number)
                {
                    return Some(111 + number);
                }
                if let Some(suffix) = self.code.strip_prefix("Numpad")
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

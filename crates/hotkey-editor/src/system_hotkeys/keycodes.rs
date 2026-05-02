pub(crate) struct KeyCodes;

impl KeyCodes {
    pub(crate) fn label(code: u32) -> String {
        match code {
            8 => String::from("Backspace"),
            9 => String::from("Tab"),
            13 => String::from("Enter"),
            16 => String::from("Shift"),
            17 => String::from("Ctrl"),
            18 => String::from("Alt"),
            19 => String::from("Pause"),
            20 => String::from("CapsLock"),
            27 => String::from("Esc"),
            32 => String::from("Space"),
            33 => String::from("PageUp"),
            34 => String::from("PageDown"),
            35 => String::from("End"),
            36 => String::from("Home"),
            37 => String::from("Left"),
            38 => String::from("Up"),
            39 => String::from("Right"),
            40 => String::from("Down"),
            45 => String::from("Insert"),
            46 => String::from("Delete"),
            48..=57 => format!("{}", code - 48),
            65..=90 => {
                let key_character = char::from_u32(code).unwrap_or('?');
                format!("{key_character}")
            }
            96..=105 => format!("Num{}", code - 96),
            106 => String::from("Num*"),
            107 => String::from("Num+"),
            109 => String::from("Num-"),
            110 => String::from("Num."),
            111 => String::from("Num/"),
            112..=123 => format!("F{}", code - 111),
            144 => String::from("NumLock"),
            145 => String::from("ScrollLock"),
            186 => String::from(";"),
            187 => String::from("="),
            188 => String::from(","),
            189 => String::from("-"),
            190 => String::from("."),
            191 => String::from("/"),
            192 => String::from("`"),
            219 => String::from("["),
            220 => String::from("\\"),
            221 => String::from("]"),
            222 => String::from("'"),
            other_code => format!("Key {other_code}"),
        }
    }

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

    pub(crate) fn modifier_prefix(modifier: Option<&str>) -> &'static str {
        match modifier.unwrap_or("").to_ascii_lowercase().as_str() {
            "alt" => "Alt + ",
            "ctrl" => "Ctrl + ",
            "ctrl_or_alt" => "Ctrl/Alt + ",
            "shift" => "Shift + ",
            _ => "",
        }
    }
}

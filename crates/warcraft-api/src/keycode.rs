use std::fmt;

pub struct KeyCode {
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

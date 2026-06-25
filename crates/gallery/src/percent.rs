/// Minimal percent codec for the one place a story id crosses a URL: the
/// preview iframe. Unreserved characters and `/` pass through; everything
/// else (spaces, punctuation) is percent-escaped so the id round-trips
/// across the iframe boundary.
pub(crate) struct Percent;

impl Percent {
    pub(crate) fn encode(value: &str) -> String {
        let mut encoded = String::with_capacity(value.len());
        for byte in value.bytes() {
            let is_unreserved =
                byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/');
            if is_unreserved {
                encoded.push(char::from(byte));
            } else {
                let high = Self::hex_digit(byte >> 4);
                let low = Self::hex_digit(byte & 0x0f);
                encoded.push('%');
                encoded.push(high);
                encoded.push(low);
            }
        }
        encoded
    }

    pub(crate) fn decode(value: &str) -> String {
        let raw = value.as_bytes();
        let mut decoded_bytes: Vec<u8> = Vec::with_capacity(raw.len());
        let mut index = 0;
        while index < raw.len() {
            let byte = raw[index];
            if byte == b'%' && index + 2 < raw.len() {
                let high = Self::hex_value(raw[index + 1]);
                let low = Self::hex_value(raw[index + 2]);
                if let (Some(high_value), Some(low_value)) = (high, low) {
                    let combined = (high_value << 4) | low_value;
                    decoded_bytes.push(combined);
                    index += 3;
                    continue;
                }
            }
            decoded_bytes.push(byte);
            index += 1;
        }
        let recovered = String::from_utf8_lossy(&decoded_bytes);
        recovered.into_owned()
    }

    fn hex_digit(nibble: u8) -> char {
        let table = b"0123456789ABCDEF";
        let resolved = table[usize::from(nibble)];
        char::from(resolved)
    }

    fn hex_value(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_leaves_unreserved_and_slash() {
        assert_eq!(Percent::encode("Buttons/Primary"), "Buttons/Primary");
    }

    #[test]
    fn encode_escapes_space() {
        assert_eq!(
            Percent::encode("Dialog header/Default"),
            "Dialog%20header/Default"
        );
    }

    #[test]
    fn decode_reverses_encode() {
        let original = "Dialog header/Default";
        let encoded = Percent::encode(original);
        assert_eq!(Percent::decode(&encoded), original);
    }

    #[test]
    fn decode_passes_plain_through() {
        assert_eq!(Percent::decode("Buttons/Primary"), "Buttons/Primary");
    }
}

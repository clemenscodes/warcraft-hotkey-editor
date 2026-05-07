pub struct WarcraftColorCodes;

impl WarcraftColorCodes {
    pub fn stripped(input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut characters = input.chars().peekable();
        while let Some(current_character) = characters.next() {
            if current_character != '|' {
                output.push(current_character);
                continue;
            }
            let Some(marker_character) = characters.next() else {
                output.push('|');
                break;
            };
            match marker_character {
                'c' | 'C' => {
                    for _ in 0..8 {
                        characters.next();
                    }
                }
                'r' | 'R' => {}
                other_character => {
                    output.push('|');
                    output.push(other_character);
                }
            }
        }
        output
    }
}

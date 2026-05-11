pub struct InnerSpaces;

impl InnerSpaces {
    pub fn collapsed(input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut previous_was_space = false;
        for current_character in input.chars() {
            let current_is_space = current_character == ' ' || current_character == '\t';
            if current_is_space {
                if previous_was_space {
                    continue;
                }
                output.push(' ');
                previous_was_space = true;
            } else {
                output.push(current_character);
                previous_was_space = false;
            }
        }
        output.trim().to_string()
    }
}

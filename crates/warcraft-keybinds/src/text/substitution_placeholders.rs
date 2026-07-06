#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SubstitutionPlaceholders;

impl SubstitutionPlaceholders {
    pub fn stripped(input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut depth: usize = 0;
        for current_character in input.chars() {
            match current_character {
                '<' => depth += 1,
                '>' if depth > 0 => {
                    depth -= 1;
                    if depth == 0 {
                        output.push('?');
                    }
                }
                other_character if depth == 0 => output.push(other_character),
                _ => {}
            }
        }
        output
    }
}

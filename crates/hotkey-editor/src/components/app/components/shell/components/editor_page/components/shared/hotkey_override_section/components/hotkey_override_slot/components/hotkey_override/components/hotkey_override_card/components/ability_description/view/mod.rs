#[derive(Clone, PartialEq)]
pub struct AbilityDescriptionView {
    pub description_lines: Vec<String>,
}

impl ddd::View for AbilityDescriptionView {}

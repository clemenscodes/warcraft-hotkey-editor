use dioxus::prelude::*;

/// The description text as pre-split lines; each becomes its own paragraph.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityDescriptionProps {
    pub description_lines: Vec<String>,
}

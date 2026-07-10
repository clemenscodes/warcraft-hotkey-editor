use super::view::AbilityDescriptionView;
use dioxus::prelude::*;

/// The description text as pre-split lines; each becomes its own paragraph.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityDescriptionProps {
    pub description_lines: Vec<String>,
}

impl From<&AbilityDescriptionView> for AbilityDescriptionProps {
    fn from(view: &AbilityDescriptionView) -> Self {
        let AbilityDescriptionView { description_lines } = view.clone();
        Self { description_lines }
    }
}

impl ddd::Props for AbilityDescriptionProps {
    type View = AbilityDescriptionView;
}

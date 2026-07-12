use super::view::AbilityDescriptionView;
use dioxus::prelude::*;

/// The description text as pre-split lines; each becomes its own paragraph.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityDescriptionModel {
    pub description_lines: Vec<String>,
}

impl From<&AbilityDescriptionView> for AbilityDescriptionModel {
    fn from(view: &AbilityDescriptionView) -> Self {
        let AbilityDescriptionView { description_lines } = view.clone();
        Self { description_lines }
    }
}

impl ddd::Model for AbilityDescriptionModel {
    type View = AbilityDescriptionView;
}

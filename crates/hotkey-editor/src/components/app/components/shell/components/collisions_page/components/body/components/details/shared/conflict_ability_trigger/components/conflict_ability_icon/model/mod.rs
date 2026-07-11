use super::view::ConflictAbilityIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictAbilityIconView> for ConflictAbilityIconModel {
    fn from(view: &ConflictAbilityIconView) -> Self {
        let ConflictAbilityIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for ConflictAbilityIconModel {
    type View = ConflictAbilityIconView;
}

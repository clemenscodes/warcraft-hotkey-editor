use super::view::ConflictAbilityIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictAbilityIconView> for ConflictAbilityIconProps {
    fn from(view: &ConflictAbilityIconView) -> Self {
        let ConflictAbilityIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for ConflictAbilityIconProps {
    type View = ConflictAbilityIconView;
}

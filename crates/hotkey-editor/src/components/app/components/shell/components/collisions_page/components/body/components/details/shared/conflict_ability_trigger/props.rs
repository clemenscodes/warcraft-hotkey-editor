use super::view::ConflictAbilityTriggerView;
use dioxus::prelude::*;

/// The icon button that opens the carrying unit: the ability icon it shows (its source
/// and alt text) and the click handler that navigates to that unit.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityTriggerProps {
    pub onclick: EventHandler<MouseEvent>,
    pub icon_src: Option<String>,
    #[props(into)]
    pub icon_alt: String,
}

impl From<&ConflictAbilityTriggerView> for ConflictAbilityTriggerProps {
    fn from(view: &ConflictAbilityTriggerView) -> Self {
        let ConflictAbilityTriggerView {
            onclick,
            icon_src,
            icon_alt,
        } = view.clone();
        Self {
            onclick,
            icon_src,
            icon_alt,
        }
    }
}

impl ddd::Props for ConflictAbilityTriggerProps {
    type View = ConflictAbilityTriggerView;
}

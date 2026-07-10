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

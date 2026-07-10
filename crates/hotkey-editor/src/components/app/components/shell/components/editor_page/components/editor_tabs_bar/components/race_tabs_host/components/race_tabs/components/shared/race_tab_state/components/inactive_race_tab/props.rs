use dioxus::prelude::*;

/// The inactive variant's props: the tab's display name and handlers, forwarded to the
/// `RaceTab` it renders. Inactive adds nothing on top — it is a named alias for the base
/// look, kept for symmetry with the active variant and for a clean dispatcher.
#[derive(Props, Clone, PartialEq)]
pub struct InactiveRaceTabProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

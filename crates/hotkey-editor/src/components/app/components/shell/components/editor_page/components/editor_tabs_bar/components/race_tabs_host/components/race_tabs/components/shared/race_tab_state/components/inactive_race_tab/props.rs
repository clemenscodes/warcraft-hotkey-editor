use super::view::InactiveRaceTabView;
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

impl From<&InactiveRaceTabView> for InactiveRaceTabProps {
    fn from(view: &InactiveRaceTabView) -> Self {
        let InactiveRaceTabView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Props for InactiveRaceTabProps {
    type View = InactiveRaceTabView;
}

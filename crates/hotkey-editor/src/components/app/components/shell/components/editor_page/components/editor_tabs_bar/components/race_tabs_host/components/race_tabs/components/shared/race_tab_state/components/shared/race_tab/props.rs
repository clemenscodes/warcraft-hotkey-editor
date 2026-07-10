use dioxus::prelude::*;

/// The base race tab's props: the display name to show, plus the pointer/keyboard
/// handlers the `<button>` needs. The active and inactive variants each build this and
/// render `RaceTab`; the label is a plain display string, and the base wraps it in its
/// own `RaceTabLabel` child.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

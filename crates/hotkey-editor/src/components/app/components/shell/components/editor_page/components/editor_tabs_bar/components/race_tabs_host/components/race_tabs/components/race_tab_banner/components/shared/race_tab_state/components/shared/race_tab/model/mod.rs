use super::view::RaceTabView;
use dioxus::prelude::*;

/// The base race tab's props: the display name to show, plus the pointer/keyboard
/// handlers the `<button>` needs. The active and inactive variants each build this and
/// render `RaceTab`; the label is a plain display string, and the base wraps it in its
/// own `RaceTabLabel` child.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabModel {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabView> for RaceTabModel {
    fn from(view: &RaceTabView) -> Self {
        let RaceTabView {
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

impl ddd::Model for RaceTabModel {
    type View = RaceTabView;
}

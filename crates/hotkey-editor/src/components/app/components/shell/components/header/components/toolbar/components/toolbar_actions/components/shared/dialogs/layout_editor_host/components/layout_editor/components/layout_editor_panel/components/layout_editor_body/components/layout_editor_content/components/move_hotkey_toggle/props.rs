use super::view::MoveHotkeyToggleView;
use dioxus::prelude::*;

/// The toggle's current value and the change handler.
#[derive(Props, Clone, PartialEq)]
pub struct MoveHotkeyToggleProps {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl From<&MoveHotkeyToggleView> for MoveHotkeyToggleProps {
    fn from(view: &MoveHotkeyToggleView) -> Self {
        let MoveHotkeyToggleView { checked, on_toggle } = view.clone();
        Self { checked, on_toggle }
    }
}

impl ddd::Props for MoveHotkeyToggleProps {
    type View = MoveHotkeyToggleView;
}

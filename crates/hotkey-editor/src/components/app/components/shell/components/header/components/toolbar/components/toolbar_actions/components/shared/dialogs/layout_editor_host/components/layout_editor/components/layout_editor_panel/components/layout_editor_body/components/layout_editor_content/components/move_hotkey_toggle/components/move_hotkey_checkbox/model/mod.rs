use super::view::MoveHotkeyCheckboxView;
use dioxus::prelude::*;

/// The checkbox's current value and the change handler forwarded from the toggle.
#[derive(Props, Clone, PartialEq)]
pub struct MoveHotkeyCheckboxModel {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl From<&MoveHotkeyCheckboxView> for MoveHotkeyCheckboxModel {
    fn from(view: &MoveHotkeyCheckboxView) -> Self {
        let MoveHotkeyCheckboxView { checked, on_toggle } = view.clone();
        Self { checked, on_toggle }
    }
}

impl ddd::Model for MoveHotkeyCheckboxModel {
    type View = MoveHotkeyCheckboxView;
}

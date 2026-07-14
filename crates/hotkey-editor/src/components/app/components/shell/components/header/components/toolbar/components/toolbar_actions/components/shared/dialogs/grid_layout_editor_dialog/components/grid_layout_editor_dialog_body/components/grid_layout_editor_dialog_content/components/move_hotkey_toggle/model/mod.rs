use super::view::MoveHotkeyToggleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoveHotkeyToggleModel {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl From<&MoveHotkeyToggleView> for MoveHotkeyToggleModel {
    fn from(view: &MoveHotkeyToggleView) -> Self {
        let MoveHotkeyToggleView { checked, on_toggle } = view.clone();
        Self { checked, on_toggle }
    }
}

impl ddd::Model for MoveHotkeyToggleModel {
    type View = MoveHotkeyToggleView;
}

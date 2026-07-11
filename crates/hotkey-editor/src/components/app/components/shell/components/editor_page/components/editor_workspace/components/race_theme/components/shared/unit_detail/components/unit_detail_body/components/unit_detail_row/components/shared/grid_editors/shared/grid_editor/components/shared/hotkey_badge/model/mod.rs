use super::state::HotkeyBadgeState;
use super::view::HotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyBadgeModel {
    pub letter: HotkeyToken,
    #[props(default)]
    pub state: HotkeyBadgeState,
}

impl From<&HotkeyBadgeView> for HotkeyBadgeModel {
    fn from(view: &HotkeyBadgeView) -> Self {
        let HotkeyBadgeView { letter, state } = view.clone();
        Self { letter, state }
    }
}

impl ddd::Model for HotkeyBadgeModel {
    type View = HotkeyBadgeView;
}

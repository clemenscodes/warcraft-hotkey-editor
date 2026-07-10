use super::state::HotkeyBadgeState;
use super::view::HotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyBadgeProps {
    pub letter: HotkeyToken,
    #[props(default)]
    pub state: HotkeyBadgeState,
}

impl From<&HotkeyBadgeView> for HotkeyBadgeProps {
    fn from(view: &HotkeyBadgeView) -> Self {
        let HotkeyBadgeView { letter, state } = view.clone();
        Self { letter, state }
    }
}

impl ddd::Props for HotkeyBadgeProps {
    type View = HotkeyBadgeView;
}

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub preview_open: Signal<bool>,
    pub layout_dialog_open: Signal<bool>,
    pub templates_dialog_open: Signal<bool>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub navigation: ViewNavigationContext,
}

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;

use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub navigation: ViewNavigationContext,
}

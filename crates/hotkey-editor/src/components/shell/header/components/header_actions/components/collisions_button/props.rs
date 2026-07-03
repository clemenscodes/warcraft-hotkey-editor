use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
}

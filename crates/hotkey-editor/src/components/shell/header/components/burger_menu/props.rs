use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
}

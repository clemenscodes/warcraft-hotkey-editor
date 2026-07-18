use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceTabBinding {
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

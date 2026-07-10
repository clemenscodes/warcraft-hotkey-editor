use super::state::ButtonVariant;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

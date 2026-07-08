use dioxus::prelude::*;

/// The action button's click handler and label text.
#[derive(Props, Clone, PartialEq)]
pub struct SecondaryButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

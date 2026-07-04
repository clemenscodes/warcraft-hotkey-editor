use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutButtonProps {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderBrandProps {
    pub onclick: EventHandler<MouseEvent>,
}

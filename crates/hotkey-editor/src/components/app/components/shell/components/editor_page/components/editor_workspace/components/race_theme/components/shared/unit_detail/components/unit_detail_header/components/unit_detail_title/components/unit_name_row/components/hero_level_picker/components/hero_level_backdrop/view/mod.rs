use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for HeroLevelBackdropView {}

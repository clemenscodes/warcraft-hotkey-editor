use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopeBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for RaceScopeBackdropView {}

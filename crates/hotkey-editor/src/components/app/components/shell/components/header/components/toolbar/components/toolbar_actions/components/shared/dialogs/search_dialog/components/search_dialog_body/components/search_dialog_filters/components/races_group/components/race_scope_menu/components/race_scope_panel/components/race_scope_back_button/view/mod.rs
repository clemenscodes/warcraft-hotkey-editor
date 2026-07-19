use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopeBackButtonView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for RaceScopeBackButtonView {}

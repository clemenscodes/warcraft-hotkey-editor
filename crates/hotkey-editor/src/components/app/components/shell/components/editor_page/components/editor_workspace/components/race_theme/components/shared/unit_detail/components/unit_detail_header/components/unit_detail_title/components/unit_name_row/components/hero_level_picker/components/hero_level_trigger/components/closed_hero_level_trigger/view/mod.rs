use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ClosedHeroLevelTriggerView {
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ClosedHeroLevelTriggerView {}

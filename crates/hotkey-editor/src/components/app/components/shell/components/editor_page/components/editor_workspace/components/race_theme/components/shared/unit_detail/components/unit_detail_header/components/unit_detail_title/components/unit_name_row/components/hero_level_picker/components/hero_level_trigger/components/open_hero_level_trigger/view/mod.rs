use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OpenHeroLevelTriggerView {
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for OpenHeroLevelTriggerView {}

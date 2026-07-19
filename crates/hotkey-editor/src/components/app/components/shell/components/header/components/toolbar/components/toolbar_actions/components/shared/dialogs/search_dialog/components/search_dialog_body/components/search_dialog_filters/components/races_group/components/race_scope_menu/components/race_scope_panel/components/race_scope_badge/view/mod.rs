use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Clone, PartialEq)]
pub struct RaceScopeBadgeView {
    pub race: Race,
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for RaceScopeBadgeView {}

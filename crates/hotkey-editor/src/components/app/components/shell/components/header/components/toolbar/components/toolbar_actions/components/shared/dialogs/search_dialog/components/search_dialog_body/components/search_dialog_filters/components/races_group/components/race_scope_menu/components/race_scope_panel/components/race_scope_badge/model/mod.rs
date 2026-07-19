use super::view::RaceScopeBadgeView;
use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeBadgeModel {
    pub race: Race,
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceScopeBadgeView> for RaceScopeBadgeModel {
    fn from(view: &RaceScopeBadgeView) -> Self {
        let RaceScopeBadgeView {
            race,
            is_active,
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            race,
            is_active,
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for RaceScopeBadgeModel {
    type View = RaceScopeBadgeView;
}

use super::view::ClosedHeroLevelTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClosedHeroLevelTriggerModel {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ClosedHeroLevelTriggerView> for ClosedHeroLevelTriggerModel {
    fn from(view: &ClosedHeroLevelTriggerView) -> Self {
        let ClosedHeroLevelTriggerView { number, onclick } = view.clone();
        Self { number, onclick }
    }
}

impl ddd::Model for ClosedHeroLevelTriggerModel {
    type View = ClosedHeroLevelTriggerView;
}

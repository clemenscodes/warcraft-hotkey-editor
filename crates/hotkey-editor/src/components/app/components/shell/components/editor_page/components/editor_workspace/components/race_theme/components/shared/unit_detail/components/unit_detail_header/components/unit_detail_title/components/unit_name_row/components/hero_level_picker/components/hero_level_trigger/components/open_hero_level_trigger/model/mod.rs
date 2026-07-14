use super::view::OpenHeroLevelTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OpenHeroLevelTriggerModel {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&OpenHeroLevelTriggerView> for OpenHeroLevelTriggerModel {
    fn from(view: &OpenHeroLevelTriggerView) -> Self {
        let OpenHeroLevelTriggerView { number, onclick } = view.clone();
        Self { number, onclick }
    }
}

impl ddd::Model for OpenHeroLevelTriggerModel {
    type View = OpenHeroLevelTriggerView;
}

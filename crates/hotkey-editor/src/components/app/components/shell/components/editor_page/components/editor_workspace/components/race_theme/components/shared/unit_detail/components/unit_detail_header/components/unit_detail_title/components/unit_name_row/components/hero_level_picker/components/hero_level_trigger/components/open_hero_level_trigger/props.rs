use super::view::OpenHeroLevelTriggerView;
use dioxus::prelude::*;

/// The open-look trigger's inputs: the current level caption and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct OpenHeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&OpenHeroLevelTriggerView> for OpenHeroLevelTriggerProps {
    fn from(view: &OpenHeroLevelTriggerView) -> Self {
        let OpenHeroLevelTriggerView { number, onclick } = view.clone();
        Self { number, onclick }
    }
}

impl ddd::Props for OpenHeroLevelTriggerProps {
    type View = OpenHeroLevelTriggerView;
}

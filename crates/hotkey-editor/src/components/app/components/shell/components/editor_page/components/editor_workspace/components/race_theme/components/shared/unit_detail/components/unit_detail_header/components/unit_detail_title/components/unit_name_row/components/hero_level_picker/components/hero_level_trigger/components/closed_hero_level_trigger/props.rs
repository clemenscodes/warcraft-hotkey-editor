use super::view::ClosedHeroLevelTriggerView;
use dioxus::prelude::*;

/// The resting-look trigger's inputs: the current level caption and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct ClosedHeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ClosedHeroLevelTriggerView> for ClosedHeroLevelTriggerProps {
    fn from(view: &ClosedHeroLevelTriggerView) -> Self {
        let ClosedHeroLevelTriggerView { number, onclick } = view.clone();
        Self { number, onclick }
    }
}

impl ddd::Props for ClosedHeroLevelTriggerProps {
    type View = ClosedHeroLevelTriggerView;
}

use super::view::RaceScopeTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeTriggerModel {
    #[props(into)]
    pub summary: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&RaceScopeTriggerView> for RaceScopeTriggerModel {
    fn from(view: &RaceScopeTriggerView) -> Self {
        let RaceScopeTriggerView {
            summary,
            is_open,
            onclick,
        } = view.clone();
        Self {
            summary,
            is_open,
            onclick,
        }
    }
}

impl ddd::Model for RaceScopeTriggerModel {
    type View = RaceScopeTriggerView;
}

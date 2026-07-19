use super::view::FilterTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterTriggerModel {
    pub active: bool,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&FilterTriggerView> for FilterTriggerModel {
    fn from(view: &FilterTriggerView) -> Self {
        let FilterTriggerView {
            active,
            count,
            onclick,
        } = view.clone();
        Self {
            active,
            count,
            onclick,
        }
    }
}

impl ddd::Model for FilterTriggerModel {
    type View = FilterTriggerView;
}

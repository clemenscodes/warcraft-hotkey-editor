use super::view::IdleFilterTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleFilterTriggerModel {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleFilterTriggerView> for IdleFilterTriggerModel {
    fn from(view: &IdleFilterTriggerView) -> Self {
        let IdleFilterTriggerView { count, onclick } = view.clone();
        Self { count, onclick }
    }
}

impl ddd::Model for IdleFilterTriggerModel {
    type View = IdleFilterTriggerView;
}

use super::view::ActiveFilterTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveFilterTriggerModel {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveFilterTriggerView> for ActiveFilterTriggerModel {
    fn from(view: &ActiveFilterTriggerView) -> Self {
        let ActiveFilterTriggerView { count, onclick } = view.clone();
        Self { count, onclick }
    }
}

impl ddd::Model for ActiveFilterTriggerModel {
    type View = ActiveFilterTriggerView;
}

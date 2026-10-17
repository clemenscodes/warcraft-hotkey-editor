use super::view::ClosedSearchScopeTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClosedSearchScopeTriggerModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ClosedSearchScopeTriggerView> for ClosedSearchScopeTriggerModel {
    fn from(view: &ClosedSearchScopeTriggerView) -> Self {
        let ClosedSearchScopeTriggerView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Layered for ClosedSearchScopeTriggerModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ClosedSearchScopeTriggerModel {
    type View = ClosedSearchScopeTriggerView;
}

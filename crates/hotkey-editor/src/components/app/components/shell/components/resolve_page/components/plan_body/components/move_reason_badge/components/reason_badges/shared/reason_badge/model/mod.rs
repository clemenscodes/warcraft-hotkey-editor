use super::view::ReasonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&ReasonBadgeView> for ReasonBadgeModel {
    fn from(view: &ReasonBadgeView) -> Self {
        let ReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Layered for ReasonBadgeModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ReasonBadgeModel {
    type View = ReasonBadgeView;
}

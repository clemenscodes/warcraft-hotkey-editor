use super::view::StuckReasonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StuckReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&StuckReasonBadgeView> for StuckReasonBadgeModel {
    fn from(view: &StuckReasonBadgeView) -> Self {
        let StuckReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Layered for StuckReasonBadgeModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for StuckReasonBadgeModel {
    type View = StuckReasonBadgeView;
}

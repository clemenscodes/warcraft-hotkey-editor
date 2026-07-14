use super::view::SwapReasonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwapReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&SwapReasonBadgeView> for SwapReasonBadgeModel {
    fn from(view: &SwapReasonBadgeView) -> Self {
        let SwapReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for SwapReasonBadgeModel {
    type View = SwapReasonBadgeView;
}

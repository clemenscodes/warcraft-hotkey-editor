use super::view::SpillReasonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SpillReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&SpillReasonBadgeView> for SpillReasonBadgeModel {
    fn from(view: &SpillReasonBadgeView) -> Self {
        let SpillReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for SpillReasonBadgeModel {
    type View = SpillReasonBadgeView;
}

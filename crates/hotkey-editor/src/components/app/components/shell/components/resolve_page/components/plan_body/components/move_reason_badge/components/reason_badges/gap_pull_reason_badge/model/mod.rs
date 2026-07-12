use super::view::GapPullReasonBadgeView;
use dioxus::prelude::*;

/// The "GapPull" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct GapPullReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&GapPullReasonBadgeView> for GapPullReasonBadgeModel {
    fn from(view: &GapPullReasonBadgeView) -> Self {
        let GapPullReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for GapPullReasonBadgeModel {
    type View = GapPullReasonBadgeView;
}

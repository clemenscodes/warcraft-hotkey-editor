use super::view::GapPullReasonBadgeView;
use dioxus::prelude::*;

/// The "GapPull" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct GapPullReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&GapPullReasonBadgeView> for GapPullReasonBadgeProps {
    fn from(view: &GapPullReasonBadgeView) -> Self {
        let GapPullReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for GapPullReasonBadgeProps {
    type View = GapPullReasonBadgeView;
}

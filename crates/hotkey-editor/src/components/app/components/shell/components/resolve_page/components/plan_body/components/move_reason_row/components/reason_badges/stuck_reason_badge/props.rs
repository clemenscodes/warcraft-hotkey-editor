use super::view::StuckReasonBadgeView;
use dioxus::prelude::*;

/// The "Stuck" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct StuckReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&StuckReasonBadgeView> for StuckReasonBadgeProps {
    fn from(view: &StuckReasonBadgeView) -> Self {
        let StuckReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for StuckReasonBadgeProps {
    type View = StuckReasonBadgeView;
}

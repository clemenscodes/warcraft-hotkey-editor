use super::view::SpillReasonBadgeView;
use dioxus::prelude::*;

/// The "Spill" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct SpillReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&SpillReasonBadgeView> for SpillReasonBadgeProps {
    fn from(view: &SpillReasonBadgeView) -> Self {
        let SpillReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for SpillReasonBadgeProps {
    type View = SpillReasonBadgeView;
}

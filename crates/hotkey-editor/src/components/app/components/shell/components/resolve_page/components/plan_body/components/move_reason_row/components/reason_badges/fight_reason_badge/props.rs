use super::view::FightReasonBadgeView;
use dioxus::prelude::*;

/// The "Fight" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct FightReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&FightReasonBadgeView> for FightReasonBadgeProps {
    fn from(view: &FightReasonBadgeView) -> Self {
        let FightReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for FightReasonBadgeProps {
    type View = FightReasonBadgeView;
}

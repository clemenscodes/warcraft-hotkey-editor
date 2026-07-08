use super::super::reason_badge::{ReasonBadgeColor, ReasonBadgeProps};
use dioxus::prelude::*;

/// The "Gap pull" reason badge's props: its label text. The colour is bound to
/// `ReasonBadgeColor::Success` by the conversion below.
#[derive(Props, Clone, PartialEq)]
pub struct GapPullBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&GapPullBadgeProps> for ReasonBadgeProps {
    fn from(props: &GapPullBadgeProps) -> Self {
        let label = props.label.clone();
        Self {
            color: ReasonBadgeColor::Success,
            label,
        }
    }
}

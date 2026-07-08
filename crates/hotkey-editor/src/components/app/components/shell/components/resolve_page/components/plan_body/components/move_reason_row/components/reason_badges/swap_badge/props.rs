use super::super::reason_badge::{ReasonBadgeColor, ReasonBadgeProps};
use dioxus::prelude::*;

/// The "Swap" reason badge's props: its label text. The colour is bound to
/// `ReasonBadgeColor::Undead` by the conversion below.
#[derive(Props, Clone, PartialEq)]
pub struct SwapBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&SwapBadgeProps> for ReasonBadgeProps {
    fn from(props: &SwapBadgeProps) -> Self {
        let label = props.label.clone();
        Self {
            color: ReasonBadgeColor::Undead,
            label,
        }
    }
}

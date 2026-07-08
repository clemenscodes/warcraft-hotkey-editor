use super::super::reason_badge::{ReasonBadgeColor, ReasonBadgeProps};
use dioxus::prelude::*;

/// The "Spill" reason badge's props: its label text. The colour is bound to
/// `ReasonBadgeColor::Human` by the conversion below.
#[derive(Props, Clone, PartialEq)]
pub struct SpillBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&SpillBadgeProps> for ReasonBadgeProps {
    fn from(props: &SpillBadgeProps) -> Self {
        let label = props.label.clone();
        Self {
            color: ReasonBadgeColor::Human,
            label,
        }
    }
}

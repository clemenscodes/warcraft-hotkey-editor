use super::super::reason_badge::{ReasonBadgeColor, ReasonBadgeProps};
use dioxus::prelude::*;

/// The "Stuck" reason badge's props: its label text. The colour is bound to
/// `ReasonBadgeColor::Orc` by the conversion below.
#[derive(Props, Clone, PartialEq)]
pub struct StuckBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&StuckBadgeProps> for ReasonBadgeProps {
    fn from(props: &StuckBadgeProps) -> Self {
        let label = props.label.clone();
        Self {
            color: ReasonBadgeColor::Orc,
            label,
        }
    }
}

use super::super::reason_badge::{ReasonBadgeColor, ReasonBadgeProps};
use dioxus::prelude::*;

/// The "Fight" reason badge's props: its label text. The colour is bound to
/// `ReasonBadgeColor::Orc` by the conversion below.
#[derive(Props, Clone, PartialEq)]
pub struct FightBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&FightBadgeProps> for ReasonBadgeProps {
    fn from(props: &FightBadgeProps) -> Self {
        let label = props.label.clone();
        Self {
            color: ReasonBadgeColor::Orc,
            label,
        }
    }
}

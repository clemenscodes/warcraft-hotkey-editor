use dioxus::prelude::*;

/// The shared move-card shell; `is_stuck` tints the border for unresolved cards.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveMoveCardProps {
    #[props(default)]
    pub is_stuck: bool,
    pub children: Element,
}

use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A key already taken by another binding. Its button attributes and children are shared
/// with the other looks (they arrive as named fields from the dispatcher); this look adds
/// only the conflict color styling in its own `style.rs`, and its tooltip carries the
/// "already used by" message.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictKeyProps {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

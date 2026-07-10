use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A free, pickable key. Its button attributes and children are shared with the other
/// looks (they arrive as named fields from the dispatcher); this look adds only the
/// available color styling in its own `style.rs`.
#[derive(Props, Clone, PartialEq)]
pub struct AvailableKeyProps {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

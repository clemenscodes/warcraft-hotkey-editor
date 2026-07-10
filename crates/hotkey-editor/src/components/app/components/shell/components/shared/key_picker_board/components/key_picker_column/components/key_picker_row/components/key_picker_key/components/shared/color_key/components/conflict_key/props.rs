use super::view::ConflictKeyView;
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

impl From<&ConflictKeyView> for ConflictKeyProps {
    fn from(view: &ConflictKeyView) -> Self {
        let ConflictKeyView {
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        } = view.clone();
        Self {
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        }
    }
}

impl ddd::Props for ConflictKeyProps {
    type View = ConflictKeyView;
}

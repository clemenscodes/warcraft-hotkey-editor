use super::view::ConflictKeyChipView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The red chip look's inputs: the key label, the edit-click handler, and the
/// conflict tooltip's text and placement.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictKeyChipProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl From<&ConflictKeyChipView> for ConflictKeyChipProps {
    fn from(view: &ConflictKeyChipView) -> Self {
        let ConflictKeyChipView {
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        } = view.clone();
        Self {
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        }
    }
}

impl ddd::Props for ConflictKeyChipProps {
    type View = ConflictKeyChipView;
}

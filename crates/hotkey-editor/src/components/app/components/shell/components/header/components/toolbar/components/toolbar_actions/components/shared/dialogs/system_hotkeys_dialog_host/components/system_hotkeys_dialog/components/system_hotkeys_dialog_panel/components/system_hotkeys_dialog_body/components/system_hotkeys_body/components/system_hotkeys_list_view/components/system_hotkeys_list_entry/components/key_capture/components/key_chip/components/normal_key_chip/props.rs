use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The gold chip look's inputs: the key label, the edit-click handler, and the
/// conflict tooltip's text and placement.
#[derive(Props, Clone, PartialEq)]
pub struct NormalKeyChipProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

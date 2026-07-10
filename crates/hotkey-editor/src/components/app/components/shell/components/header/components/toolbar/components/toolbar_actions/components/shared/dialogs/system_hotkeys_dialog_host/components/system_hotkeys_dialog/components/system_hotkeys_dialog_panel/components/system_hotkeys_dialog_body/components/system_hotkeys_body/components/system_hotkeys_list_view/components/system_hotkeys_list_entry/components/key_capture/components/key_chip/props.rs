use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The presentational inputs both chip looks share: the domain conflict fact that
/// selects the look, the key label to draw, the edit-click handler, and the conflict
/// tooltip's text and placement.
#[derive(Props, Clone, PartialEq)]
pub struct KeyChipProps {
    pub conflict: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

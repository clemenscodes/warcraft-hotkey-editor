use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// The presentational inputs both chip looks share: the domain conflict fact that
/// selects the look, the key label to draw, the edit-click handler, and the conflict
/// tooltip.
#[derive(Props, Clone, PartialEq)]
pub struct KeyChipProps {
    pub conflict: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

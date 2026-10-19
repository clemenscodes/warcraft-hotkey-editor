use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ConflictKeyChipView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl ddd::Layered for ConflictKeyChipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictKeyChipView {}

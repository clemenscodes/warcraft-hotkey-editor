use super::view::HighlightedSlotView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HighlightedSlotModel {
    pub slot_label: String,
    pub key_label: String,
    pub conflict: bool,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub dragging: bool,
}

impl From<&HighlightedSlotView> for HighlightedSlotModel {
    fn from(view: &HighlightedSlotView) -> Self {
        let HighlightedSlotView {
            slot_label,
            key_label,
            conflict,
            tooltip_text,
            tooltip_placement,
            dragging,
        } = view.clone();
        Self {
            slot_label,
            key_label,
            conflict,
            tooltip_text,
            tooltip_placement,
            dragging,
        }
    }
}

impl ddd::Model for HighlightedSlotModel {
    type View = HighlightedSlotView;
}

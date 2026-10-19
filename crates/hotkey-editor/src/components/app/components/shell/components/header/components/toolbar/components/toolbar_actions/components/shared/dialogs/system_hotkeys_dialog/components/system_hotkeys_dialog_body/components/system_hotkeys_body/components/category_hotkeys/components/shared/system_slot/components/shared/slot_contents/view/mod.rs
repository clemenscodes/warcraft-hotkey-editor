use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;

#[derive(Clone, PartialEq)]
pub struct SlotContentsView {
    pub slot_label: String,
    pub key_label: String,
    pub conflict: bool,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub dragging: bool,
}

impl ddd::Layered for SlotContentsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SlotContentsView {}

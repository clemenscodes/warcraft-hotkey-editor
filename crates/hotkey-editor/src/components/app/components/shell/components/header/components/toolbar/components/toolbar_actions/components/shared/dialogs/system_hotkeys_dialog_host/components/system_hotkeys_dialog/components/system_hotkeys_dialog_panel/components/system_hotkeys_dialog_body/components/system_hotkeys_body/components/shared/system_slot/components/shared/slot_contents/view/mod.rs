use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;

/// The published `View` contract mirroring [`SlotContentsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SlotContentsView {
    pub slot_label: String,
    pub key_label: String,
    pub conflict: bool,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub dragging: bool,
}

impl ddd::View for SlotContentsView {}

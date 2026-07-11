use super::view::ConflictSlotView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The conflict-look slot's props: the caption and bound-key labels, the conflict flag
/// (which reddens the key and supplies the tooltip), where the tooltip sits, and the
/// `dragging` flag (this slot is being dragged, so its contents unmount). Set field by
/// field by the dispatcher from the shared slot data.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictSlotModel {
    pub slot_label: String,
    pub key_label: String,
    pub conflict: bool,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub dragging: bool,
}

impl From<&ConflictSlotView> for ConflictSlotModel {
    fn from(view: &ConflictSlotView) -> Self {
        let ConflictSlotView {
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

impl ddd::Model for ConflictSlotModel {
    type View = ConflictSlotView;
}

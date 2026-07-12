use super::view::SlotContentsView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The framed slot's inner content: the caption and bound-key labels, the conflict
/// flag (which reddens the key and supplies the tooltip), where the tooltip sits, and
/// the `dragging` flag. While the slot is being dragged the content is unmounted (not
/// merely hidden), so the frame's root DOM node stays mounted and the pointer-capture
/// drag on the host is never interrupted by a class swap on a live element.
#[derive(Props, Clone, PartialEq)]
pub struct SlotContentsModel {
    pub slot_label: String,
    pub key_label: String,
    pub conflict: bool,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub dragging: bool,
}

impl From<&SlotContentsView> for SlotContentsModel {
    fn from(view: &SlotContentsView) -> Self {
        let SlotContentsView {
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

impl ddd::Model for SlotContentsModel {
    type View = SlotContentsView;
}

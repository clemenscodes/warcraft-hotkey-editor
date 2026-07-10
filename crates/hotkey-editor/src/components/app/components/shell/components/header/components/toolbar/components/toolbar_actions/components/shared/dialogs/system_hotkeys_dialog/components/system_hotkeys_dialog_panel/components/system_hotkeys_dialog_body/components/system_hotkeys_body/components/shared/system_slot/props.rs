use super::state::SystemSlotState;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// One framed WC3 slot cell, presentational: its glow state, caption and bound-key
/// labels, whether it is in a conflict (which reddens the key and supplies the
/// tooltip), where the tooltip sits, and the two orthogonal flags — `compact` (the
/// tighter control-group cell) and `dragging` (this slot is being dragged, so its
/// contents hide). Props in, markup out: the host shapes these and the cell renders.
#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotProps {
    pub state: SystemSlotState,
    #[props(into)]
    pub slot_label: String,
    #[props(into)]
    pub key_label: String,
    pub conflict: bool,
    #[props(into)]
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    #[props(default = false)]
    pub compact: bool,
    #[props(default = false)]
    pub dragging: bool,
}

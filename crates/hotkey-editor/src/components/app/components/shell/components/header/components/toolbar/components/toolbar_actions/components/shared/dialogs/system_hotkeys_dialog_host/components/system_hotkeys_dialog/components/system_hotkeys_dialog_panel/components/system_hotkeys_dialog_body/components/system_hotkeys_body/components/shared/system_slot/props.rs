use super::state::SystemSlotState;
use super::view::SystemSlotView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// One framed WC3 slot cell, presentational: its glow state, caption and bound-key
/// labels, whether it is in a conflict (which reddens the key and supplies the
/// tooltip), where the tooltip sits, and the `dragging` flag (this slot is being
/// dragged, so its contents unmount). The tighter control-group density is inherited
/// from the parent size container, so no density flag rides here. Props in, markup
/// out: the host shapes these and the cell renders.
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
    pub dragging: bool,
}

impl From<&SystemSlotView> for SystemSlotProps {
    fn from(view: &SystemSlotView) -> Self {
        let SystemSlotView {
            state,
            slot_label,
            key_label,
            conflict,
            tooltip_text,
            tooltip_placement,
            dragging,
        } = view.clone();
        Self {
            state,
            slot_label,
            key_label,
            conflict,
            tooltip_text,
            tooltip_placement,
            dragging,
        }
    }
}

impl ddd::Props for SystemSlotProps {
    type View = SystemSlotView;
}

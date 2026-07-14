use super::state::SystemSlotState;
use super::view::SystemSlotView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotModel {
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

impl From<&SystemSlotView> for SystemSlotModel {
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

impl ddd::Model for SystemSlotModel {
    type View = SystemSlotView;
}

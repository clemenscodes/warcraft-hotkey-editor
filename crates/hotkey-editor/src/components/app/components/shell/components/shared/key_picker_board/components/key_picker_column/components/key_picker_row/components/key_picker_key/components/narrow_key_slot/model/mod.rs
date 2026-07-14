use super::view::NarrowKeySlotView;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NarrowKeySlotModel {
    pub state: ColorKeyState,
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&NarrowKeySlotView> for NarrowKeySlotModel {
    fn from(view: &NarrowKeySlotView) -> Self {
        let NarrowKeySlotView {
            state,
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        } = view.clone();
        Self {
            state,
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        }
    }
}

impl ddd::Model for NarrowKeySlotModel {
    type View = NarrowKeySlotView;
}

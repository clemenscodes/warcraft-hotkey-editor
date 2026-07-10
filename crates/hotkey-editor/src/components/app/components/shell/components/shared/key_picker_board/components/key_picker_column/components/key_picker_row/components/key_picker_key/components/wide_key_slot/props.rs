use super::view::WideKeySlotView;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A wide-width picker key: the color state and the shared button attributes (cap label,
/// disabled flag, click handler) plus the conflict tooltip's three domain fields, all
/// passed straight through to the color leaf this slot sizes. Oversized caps
/// (`Space`, `Backspace`, the mouse side buttons) get this wider width.
#[derive(Props, Clone, PartialEq)]
pub struct WideKeySlotProps {
    pub state: ColorKeyState,
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&WideKeySlotView> for WideKeySlotProps {
    fn from(view: &WideKeySlotView) -> Self {
        let WideKeySlotView {
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

impl ddd::Props for WideKeySlotProps {
    type View = WideKeySlotView;
}

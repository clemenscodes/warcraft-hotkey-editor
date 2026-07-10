use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// A standard-width picker key: the color state and the shared button attributes
/// (cap label, `data-label` selector hook, disabled flag, click handler, conflict
/// tooltip), all passed straight through to the color leaf this slot sizes.
#[derive(Props, Clone, PartialEq)]
pub struct NarrowKeySlotProps {
    pub state: ColorKeyState,
    pub label: String,
    pub data_label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

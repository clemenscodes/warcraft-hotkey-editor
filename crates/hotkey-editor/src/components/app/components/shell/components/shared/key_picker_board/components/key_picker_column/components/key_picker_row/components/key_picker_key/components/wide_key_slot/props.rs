use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// A wide-width picker key: the color state and the shared button attributes (cap
/// label, disabled flag, click handler, conflict tooltip), all passed straight through
/// to the color leaf this slot sizes. Oversized caps
/// (`Space`, `Backspace`, the mouse side buttons) get this wider width.
#[derive(Props, Clone, PartialEq)]
pub struct WideKeySlotProps {
    pub state: ColorKeyState,
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

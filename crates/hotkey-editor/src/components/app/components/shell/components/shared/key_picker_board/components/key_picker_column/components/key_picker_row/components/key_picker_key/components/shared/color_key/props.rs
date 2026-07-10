use super::state::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// A picker key's color look plus the shared button attributes and conflict tooltip
/// every color renders the same way (the cap label, the `data-label` selector hook,
/// the disabled flag, the click handler, and the conflict tooltip). Built by the slot
/// from its own props; the three colors differ only in their own styling, and the
/// width is owned by the slot, not here.
#[derive(Props, Clone, PartialEq)]
pub struct ColorKeyProps {
    pub state: ColorKeyState,
    pub label: String,
    pub data_label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

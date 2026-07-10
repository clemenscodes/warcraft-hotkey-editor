use super::state::ColorKeyState;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A picker key's color look plus the shared button attributes (cap label, disabled flag,
/// click handler) and the conflict tooltip's three domain fields every color renders the
/// same way. Handed down by the slot from named fields; the three colors differ only in
/// their own styling, and the width is owned by the slot, not here.
#[derive(Props, Clone, PartialEq)]
pub struct ColorKeyProps {
    pub state: ColorKeyState,
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

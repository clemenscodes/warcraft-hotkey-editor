use super::state::ColorKeyState;
use super::view::ColorKeyView;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A picker key's color look plus the shared button attributes (cap label, disabled flag,
/// click handler) and the conflict tooltip's three domain fields every color renders the
/// same way. Handed down by the slot from named fields; the three colors differ only in
/// their own styling, and the width is owned by the slot, not here.
#[derive(Props, Clone, PartialEq)]
pub struct ColorKeyModel {
    pub state: ColorKeyState,
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&ColorKeyView> for ColorKeyModel {
    fn from(view: &ColorKeyView) -> Self {
        let ColorKeyView {
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

impl ddd::Model for ColorKeyModel {
    type View = ColorKeyView;
}

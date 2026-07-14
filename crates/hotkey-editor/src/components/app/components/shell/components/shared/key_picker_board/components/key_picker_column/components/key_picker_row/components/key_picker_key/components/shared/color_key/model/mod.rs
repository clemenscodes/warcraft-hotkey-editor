use super::state::ColorKeyState;
use super::view::ColorKeyView;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

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

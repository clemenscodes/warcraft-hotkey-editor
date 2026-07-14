use super::view::KeyChipView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct KeyChipModel {
    pub conflict: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl From<&KeyChipView> for KeyChipModel {
    fn from(view: &KeyChipView) -> Self {
        let KeyChipView {
            conflict,
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        } = view.clone();
        Self {
            conflict,
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        }
    }
}

impl ddd::Model for KeyChipModel {
    type View = KeyChipView;
}

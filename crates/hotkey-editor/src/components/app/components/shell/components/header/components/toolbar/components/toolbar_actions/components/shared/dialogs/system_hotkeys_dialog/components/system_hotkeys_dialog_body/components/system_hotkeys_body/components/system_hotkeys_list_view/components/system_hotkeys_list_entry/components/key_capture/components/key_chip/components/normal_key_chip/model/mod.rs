use super::view::NormalKeyChipView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NormalKeyChipModel {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl From<&NormalKeyChipView> for NormalKeyChipModel {
    fn from(view: &NormalKeyChipView) -> Self {
        let NormalKeyChipView {
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        } = view.clone();
        Self {
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        }
    }
}

impl ddd::Model for NormalKeyChipModel {
    type View = NormalKeyChipView;
}

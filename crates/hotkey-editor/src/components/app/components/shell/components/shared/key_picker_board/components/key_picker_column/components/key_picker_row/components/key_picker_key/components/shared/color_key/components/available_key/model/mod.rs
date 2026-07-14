use super::view::AvailableKeyView;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvailableKeyModel {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&AvailableKeyView> for AvailableKeyModel {
    fn from(view: &AvailableKeyView) -> Self {
        let AvailableKeyView {
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        } = view.clone();
        Self {
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
        }
    }
}

impl ddd::Model for AvailableKeyModel {
    type View = AvailableKeyView;
}

use super::view::ConflictKeyView;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictKeyModel {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&ConflictKeyView> for ConflictKeyModel {
    fn from(view: &ConflictKeyView) -> Self {
        let ConflictKeyView {
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

impl ddd::Model for ConflictKeyModel {
    type View = ConflictKeyView;
}

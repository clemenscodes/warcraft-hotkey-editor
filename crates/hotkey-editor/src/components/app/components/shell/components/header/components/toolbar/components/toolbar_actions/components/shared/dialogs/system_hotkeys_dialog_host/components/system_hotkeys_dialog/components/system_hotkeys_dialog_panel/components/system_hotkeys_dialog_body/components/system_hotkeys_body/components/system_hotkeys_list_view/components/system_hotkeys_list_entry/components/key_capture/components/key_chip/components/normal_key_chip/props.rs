use super::view::NormalKeyChipView;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The gold chip look's inputs: the key label, the edit-click handler, and the
/// conflict tooltip's text and placement.
#[derive(Props, Clone, PartialEq)]
pub struct NormalKeyChipProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl From<&NormalKeyChipView> for NormalKeyChipProps {
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

impl ddd::Props for NormalKeyChipProps {
    type View = NormalKeyChipView;
}

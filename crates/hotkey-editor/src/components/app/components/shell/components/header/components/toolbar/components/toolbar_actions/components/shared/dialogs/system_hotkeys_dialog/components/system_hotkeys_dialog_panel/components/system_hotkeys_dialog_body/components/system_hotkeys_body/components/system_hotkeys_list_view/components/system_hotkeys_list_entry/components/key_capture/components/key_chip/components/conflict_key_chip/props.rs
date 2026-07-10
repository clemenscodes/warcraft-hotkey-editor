use super::super::super::KeyChipProps;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// The red chip look's inputs: the key label, the edit-click handler, and the
/// conflict tooltip. Copied from the shared `KeyChipProps`.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictKeyChipProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

impl From<&KeyChipProps> for ConflictKeyChipProps {
    fn from(props: &KeyChipProps) -> Self {
        let label = props.label.clone();
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            label,
            onclick,
            tooltip,
        }
    }
}

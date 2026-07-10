use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`KeyChipProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyChipView {
    pub conflict: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
}

impl ddd::View for KeyChipView {}

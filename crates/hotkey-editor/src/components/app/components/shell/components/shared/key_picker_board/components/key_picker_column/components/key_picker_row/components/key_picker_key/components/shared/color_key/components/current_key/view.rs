use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CurrentKeyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CurrentKeyView {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl ddd::View for CurrentKeyView {}

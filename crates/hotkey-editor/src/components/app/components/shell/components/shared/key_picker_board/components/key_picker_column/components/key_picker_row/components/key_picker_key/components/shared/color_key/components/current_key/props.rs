use super::view::CurrentKeyView;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// The key currently bound here. Its button attributes and children are shared with the
/// other looks (they arrive as named fields from the dispatcher); this look adds only the
/// current color styling in its own `style.rs`.
#[derive(Props, Clone, PartialEq)]
pub struct CurrentKeyProps {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip_text: String,
    pub tooltip_placement: TooltipPlacement,
    pub tooltip_anchor: TooltipAnchor,
}

impl From<&CurrentKeyView> for CurrentKeyProps {
    fn from(view: &CurrentKeyView) -> Self {
        let CurrentKeyView {
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

impl ddd::Props for CurrentKeyProps {
    type View = CurrentKeyView;
}

use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// The key currently bound here. Its button attributes and children are shared with the
/// other looks (they arrive already shaped from the dispatcher); this look adds only the
/// current color styling in its own `style.rs`.
#[derive(Props, Clone, PartialEq)]
pub struct CurrentKeyProps {
    pub label: String,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub tooltip: TooltipProps,
}

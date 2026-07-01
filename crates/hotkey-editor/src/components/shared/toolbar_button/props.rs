use dioxus::prelude::*;

/// Consumers swap only the icon, click handler, and aria/disabled state; the look is
/// fixed.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    pub icon: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

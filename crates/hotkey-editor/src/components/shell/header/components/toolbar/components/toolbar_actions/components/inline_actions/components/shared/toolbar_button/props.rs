use super::components::toolbar_button_surface::ToolbarButtonSurfaceProps;
use dioxus::prelude::*;

/// Consumers swap only the icon, click handler, and aria/disabled state; the look is
/// fixed. Every attribute is a typed field so callers build these props by
/// conversion and spread them, rather than passing loose attributes by hand.
#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub data_action: Option<&'static str>,
    #[props(default)]
    pub aria_haspopup: Option<&'static str>,
    #[props(default)]
    pub aria_expanded: Option<bool>,
    #[props(default)]
    pub aria_pressed: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ToolbarButtonProps> for ToolbarButtonSurfaceProps {
    fn from(props: &ToolbarButtonProps) -> Self {
        let icon = props.icon;
        let aria_label = props.aria_label;
        let disabled = props.disabled;
        let data_action = props.data_action;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let onclick = props.onclick;
        Self {
            icon,
            aria_label,
            disabled,
            data_action,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            onclick,
        }
    }
}

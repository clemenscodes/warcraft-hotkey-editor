use super::components::toolbar_button_icon::ToolbarButtonIconProps;
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

impl From<&ToolbarButtonProps> for ToolbarButtonIconProps {
    fn from(props: &ToolbarButtonProps) -> Self {
        let icon = props.icon;
        Self { icon }
    }
}

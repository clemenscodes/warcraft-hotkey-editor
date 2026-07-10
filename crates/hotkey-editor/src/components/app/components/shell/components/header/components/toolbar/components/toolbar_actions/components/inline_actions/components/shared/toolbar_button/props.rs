use super::view::ToolbarButtonView;
use dioxus::prelude::*;

/// Consumers swap only the icon, click handler, and aria/disabled state; the look is
/// fixed. Every attribute is a typed field, set by name where the button is rendered.
#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub aria_haspopup: Option<&'static str>,
    #[props(default)]
    pub aria_expanded: Option<bool>,
    #[props(default)]
    pub aria_pressed: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ToolbarButtonView> for ToolbarButtonProps {
    fn from(view: &ToolbarButtonView) -> Self {
        let ToolbarButtonView {
            icon,
            aria_label,
            disabled,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            onclick,
        } = view.clone();
        Self {
            icon,
            aria_label,
            disabled,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            onclick,
        }
    }
}

impl ddd::Props for ToolbarButtonProps {
    type View = ToolbarButtonView;
}

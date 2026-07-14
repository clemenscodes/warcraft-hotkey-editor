use super::view::ToolbarButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonModel {
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

impl From<&ToolbarButtonView> for ToolbarButtonModel {
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

impl ddd::Model for ToolbarButtonModel {
    type View = ToolbarButtonView;
}

use super::view::ActiveMenuItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveMenuItemModel {
    pub icon: &'static str,
    pub label: String,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveMenuItemView> for ActiveMenuItemModel {
    fn from(view: &ActiveMenuItemView) -> Self {
        let ActiveMenuItemView {
            icon,
            label,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        } = view.clone();
        Self {
            icon,
            label,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }
}

impl ddd::Model for ActiveMenuItemModel {
    type View = ActiveMenuItemView;
}

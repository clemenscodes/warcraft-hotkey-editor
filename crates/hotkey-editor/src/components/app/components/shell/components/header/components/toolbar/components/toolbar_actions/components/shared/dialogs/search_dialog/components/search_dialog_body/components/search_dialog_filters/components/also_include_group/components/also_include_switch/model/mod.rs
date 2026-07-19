use super::view::AlsoIncludeSwitchView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlsoIncludeSwitchModel {
    pub label: &'static str,
    pub popover_text: &'static str,
    pub is_on: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&AlsoIncludeSwitchView> for AlsoIncludeSwitchModel {
    fn from(view: &AlsoIncludeSwitchView) -> Self {
        let AlsoIncludeSwitchView {
            label,
            popover_text,
            is_on,
            onclick,
        } = view.clone();
        Self {
            label,
            popover_text,
            is_on,
            onclick,
        }
    }
}

impl ddd::Model for AlsoIncludeSwitchModel {
    type View = AlsoIncludeSwitchView;
}

use super::view::SwitchToggleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchToggleModel {
    pub is_on: bool,
    #[props(default)]
    pub title: Option<&'static str>,
    pub aria_label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SwitchToggleView> for SwitchToggleModel {
    fn from(view: &SwitchToggleView) -> Self {
        let SwitchToggleView {
            is_on,
            title,
            aria_label,
            onclick,
        } = view.clone();
        Self {
            is_on,
            title,
            aria_label,
            onclick,
        }
    }
}

impl ddd::Model for SwitchToggleModel {
    type View = SwitchToggleView;
}

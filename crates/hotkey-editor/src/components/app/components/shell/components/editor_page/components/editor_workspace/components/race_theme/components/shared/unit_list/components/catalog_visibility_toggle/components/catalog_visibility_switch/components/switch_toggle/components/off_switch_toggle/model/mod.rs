use super::view::OffSwitchToggleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OffSwitchToggleModel {
    pub title: Option<&'static str>,
    pub aria_label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&OffSwitchToggleView> for OffSwitchToggleModel {
    fn from(view: &OffSwitchToggleView) -> Self {
        let OffSwitchToggleView {
            title,
            aria_label,
            onclick,
        } = view.clone();
        Self {
            title,
            aria_label,
            onclick,
        }
    }
}

impl ddd::Model for OffSwitchToggleModel {
    type View = OffSwitchToggleView;
}

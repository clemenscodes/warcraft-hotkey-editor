use super::view::OnSwitchToggleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OnSwitchToggleModel {
    pub title: Option<&'static str>,
    pub aria_label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&OnSwitchToggleView> for OnSwitchToggleModel {
    fn from(view: &OnSwitchToggleView) -> Self {
        let OnSwitchToggleView {
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

impl ddd::Model for OnSwitchToggleModel {
    type View = OnSwitchToggleView;
}

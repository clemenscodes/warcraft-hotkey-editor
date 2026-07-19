use super::view::FilterSwitchView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterSwitchModel {
    pub is_on: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&FilterSwitchView> for FilterSwitchModel {
    fn from(view: &FilterSwitchView) -> Self {
        let FilterSwitchView { is_on, onclick } = view.clone();
        Self { is_on, onclick }
    }
}

impl ddd::Model for FilterSwitchModel {
    type View = FilterSwitchView;
}

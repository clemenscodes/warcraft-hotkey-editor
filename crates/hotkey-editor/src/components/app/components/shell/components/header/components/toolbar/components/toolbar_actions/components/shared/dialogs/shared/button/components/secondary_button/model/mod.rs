use super::view::SecondaryButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SecondaryButtonModel {
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

impl From<&SecondaryButtonView> for SecondaryButtonModel {
    fn from(view: &SecondaryButtonView) -> Self {
        let SecondaryButtonView { onclick, label } = view.clone();
        Self { onclick, label }
    }
}

impl ddd::Model for SecondaryButtonModel {
    type View = SecondaryButtonView;
}

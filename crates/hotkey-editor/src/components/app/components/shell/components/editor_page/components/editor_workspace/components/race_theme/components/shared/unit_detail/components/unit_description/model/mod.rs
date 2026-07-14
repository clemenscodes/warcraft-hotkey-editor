use super::view::UnitDescriptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDescriptionModel {
    #[props(into)]
    pub text: String,
}

impl From<&UnitDescriptionView> for UnitDescriptionModel {
    fn from(view: &UnitDescriptionView) -> Self {
        let UnitDescriptionView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for UnitDescriptionModel {
    type View = UnitDescriptionView;
}

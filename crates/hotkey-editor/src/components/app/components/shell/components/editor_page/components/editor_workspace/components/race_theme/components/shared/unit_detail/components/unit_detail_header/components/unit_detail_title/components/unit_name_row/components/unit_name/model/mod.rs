use super::view::UnitNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitNameModel {
    pub text: &'static str,
}

impl From<&UnitNameView> for UnitNameModel {
    fn from(view: &UnitNameView) -> Self {
        let UnitNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for UnitNameModel {
    type View = UnitNameView;
}

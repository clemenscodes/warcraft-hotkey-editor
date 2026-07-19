use super::view::ManaValueView;
use dioxus::prelude::*;
use warcraft_api::Mana;

#[derive(Props, Clone, PartialEq)]
pub struct ManaValueModel {
    pub value: Mana,
}

impl From<&ManaValueView> for ManaValueModel {
    fn from(view: &ManaValueView) -> Self {
        let ManaValueView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for ManaValueModel {
    type View = ManaValueView;
}

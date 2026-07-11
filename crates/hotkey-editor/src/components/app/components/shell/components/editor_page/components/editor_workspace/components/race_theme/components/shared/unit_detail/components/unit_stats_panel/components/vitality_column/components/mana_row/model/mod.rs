use super::view::ManaRowView;
use dioxus::prelude::*;
use warcraft_api::Mana;

/// The mana row's input: the unit's resolved mana pool at the selected level.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRowModel {
    pub value: Mana,
}

impl From<&ManaRowView> for ManaRowModel {
    fn from(view: &ManaRowView) -> Self {
        let ManaRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for ManaRowModel {
    type View = ManaRowView;
}

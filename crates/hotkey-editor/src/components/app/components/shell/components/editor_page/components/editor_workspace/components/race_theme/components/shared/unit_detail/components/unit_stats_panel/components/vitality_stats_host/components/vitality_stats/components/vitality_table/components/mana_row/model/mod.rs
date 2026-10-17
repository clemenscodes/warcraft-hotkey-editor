use super::view::ManaRowView;
use dioxus::prelude::*;
use warcraft_api::{Mana, ManaRegen};

#[derive(Props, Clone, PartialEq)]
pub struct ManaRowModel {
    pub value: Mana,
    pub regen: ManaRegen,
}

impl From<&ManaRowView> for ManaRowModel {
    fn from(view: &ManaRowView) -> Self {
        let ManaRowView { value, regen } = view.clone();
        Self { value, regen }
    }
}

impl ddd::Layered for ManaRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ManaRowModel {
    type View = ManaRowView;
}

use super::view::ArmorRowView;
use dioxus::prelude::*;
use warcraft_keybinds::Armor;

/// The armor row's input: the unit's resolved armor.
#[derive(Props, Clone, PartialEq)]
pub struct ArmorRowModel {
    pub value: Armor,
}

impl From<&ArmorRowView> for ArmorRowModel {
    fn from(view: &ArmorRowView) -> Self {
        let ArmorRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for ArmorRowModel {
    type View = ArmorRowView;
}

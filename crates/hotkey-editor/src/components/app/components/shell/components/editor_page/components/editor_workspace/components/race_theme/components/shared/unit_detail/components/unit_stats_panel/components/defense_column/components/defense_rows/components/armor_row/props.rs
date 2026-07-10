use super::view::ArmorRowView;
use dioxus::prelude::*;
use warcraft_keybinds::Armor;

/// The armor row's input: the unit's resolved armor.
#[derive(Props, Clone, PartialEq)]
pub struct ArmorRowProps {
    pub value: Armor,
}

impl From<&ArmorRowView> for ArmorRowProps {
    fn from(view: &ArmorRowView) -> Self {
        let ArmorRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for ArmorRowProps {
    type View = ArmorRowView;
}

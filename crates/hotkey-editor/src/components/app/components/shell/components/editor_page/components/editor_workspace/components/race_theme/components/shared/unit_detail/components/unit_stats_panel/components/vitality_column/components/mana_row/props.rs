use super::view::ManaRowView;
use dioxus::prelude::*;
use warcraft_keybinds::Mana;

/// The mana row's input: the unit's resolved mana pool at the selected level.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRowProps {
    pub value: Mana,
}

impl From<&ManaRowView> for ManaRowProps {
    fn from(view: &ManaRowView) -> Self {
        let ManaRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for ManaRowProps {
    type View = ManaRowView;
}

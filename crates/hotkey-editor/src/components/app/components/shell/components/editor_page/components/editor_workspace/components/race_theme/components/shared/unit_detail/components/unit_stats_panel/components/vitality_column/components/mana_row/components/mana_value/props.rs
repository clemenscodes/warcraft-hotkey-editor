use super::view::ManaValueView;
use dioxus::prelude::*;
use warcraft_keybinds::Mana;

/// The mana value leaf's input: the unit's resolved mana pool.
#[derive(Props, Clone, PartialEq)]
pub struct ManaValueProps {
    pub value: Mana,
}

impl From<&ManaValueView> for ManaValueProps {
    fn from(view: &ManaValueView) -> Self {
        let ManaValueView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for ManaValueProps {
    type View = ManaValueView;
}

use super::view::EvasionRowView;
use dioxus::prelude::*;
use warcraft_keybinds::Evasion;

/// The evasion row's input: the unit's resolved dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct EvasionRowProps {
    pub value: Evasion,
}

impl From<&EvasionRowView> for EvasionRowProps {
    fn from(view: &EvasionRowView) -> Self {
        let EvasionRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for EvasionRowProps {
    type View = EvasionRowView;
}

use super::view::EvasionRowView;
use dioxus::prelude::*;
use warcraft_api::Evasion;

/// The evasion row's input: the unit's resolved dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct EvasionRowModel {
    pub value: Evasion,
}

impl From<&EvasionRowView> for EvasionRowModel {
    fn from(view: &EvasionRowView) -> Self {
        let EvasionRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for EvasionRowModel {
    type View = EvasionRowView;
}

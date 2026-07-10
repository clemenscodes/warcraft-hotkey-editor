use super::view::DefenseTypeRowView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;

/// The defense type row's input: the unit's defense classification.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseTypeRowProps {
    pub value: DefenseType,
}

impl From<&DefenseTypeRowView> for DefenseTypeRowProps {
    fn from(view: &DefenseTypeRowView) -> Self {
        let DefenseTypeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for DefenseTypeRowProps {
    type View = DefenseTypeRowView;
}

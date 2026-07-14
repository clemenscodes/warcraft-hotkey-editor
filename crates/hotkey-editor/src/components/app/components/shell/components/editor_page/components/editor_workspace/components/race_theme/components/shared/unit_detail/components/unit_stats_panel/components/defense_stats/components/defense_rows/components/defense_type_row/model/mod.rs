use super::view::DefenseTypeRowView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;

#[derive(Props, Clone, PartialEq)]
pub struct DefenseTypeRowModel {
    pub value: DefenseType,
}

impl From<&DefenseTypeRowView> for DefenseTypeRowModel {
    fn from(view: &DefenseTypeRowView) -> Self {
        let DefenseTypeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for DefenseTypeRowModel {
    type View = DefenseTypeRowView;
}

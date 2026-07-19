use super::view::AttackTypeRowView;
use dioxus::prelude::*;
use warcraft_api::AttackType;

#[derive(Props, Clone, PartialEq)]
pub struct AttackTypeRowModel {
    pub value: AttackType,
}

impl From<&AttackTypeRowView> for AttackTypeRowModel {
    fn from(view: &AttackTypeRowView) -> Self {
        let AttackTypeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for AttackTypeRowModel {
    type View = AttackTypeRowView;
}

use super::view::DamageRowView;
use dioxus::prelude::*;
use warcraft_api::DamageRange;

#[derive(Props, Clone, PartialEq)]
pub struct DamageRowModel {
    pub value: DamageRange,
}

impl From<&DamageRowView> for DamageRowModel {
    fn from(view: &DamageRowView) -> Self {
        let DamageRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for DamageRowModel {
    type View = DamageRowView;
}

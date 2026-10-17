use super::view::DamagePerSecondRowView;
use dioxus::prelude::*;
use warcraft_api::DamagePerSecond;

#[derive(Props, Clone, PartialEq)]
pub struct DamagePerSecondRowModel {
    pub value: Option<DamagePerSecond>,
}

impl From<&DamagePerSecondRowView> for DamagePerSecondRowModel {
    fn from(view: &DamagePerSecondRowView) -> Self {
        let DamagePerSecondRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Layered for DamagePerSecondRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for DamagePerSecondRowModel {
    type View = DamagePerSecondRowView;
}

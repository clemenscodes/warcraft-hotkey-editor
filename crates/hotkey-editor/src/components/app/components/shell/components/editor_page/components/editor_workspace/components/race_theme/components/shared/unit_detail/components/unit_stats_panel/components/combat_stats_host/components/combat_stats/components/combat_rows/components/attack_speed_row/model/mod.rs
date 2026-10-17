use super::view::AttackSpeedRowView;
use dioxus::prelude::*;
use warcraft_api::AttackSpeed;

#[derive(Props, Clone, PartialEq)]
pub struct AttackSpeedRowModel {
    pub value: AttackSpeed,
}

impl From<&AttackSpeedRowView> for AttackSpeedRowModel {
    fn from(view: &AttackSpeedRowView) -> Self {
        let AttackSpeedRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Layered for AttackSpeedRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for AttackSpeedRowModel {
    type View = AttackSpeedRowView;
}

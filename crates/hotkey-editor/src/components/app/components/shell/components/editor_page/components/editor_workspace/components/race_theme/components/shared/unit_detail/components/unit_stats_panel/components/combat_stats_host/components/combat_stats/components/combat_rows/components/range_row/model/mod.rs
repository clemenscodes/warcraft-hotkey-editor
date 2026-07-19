use super::view::RangeRowView;
use dioxus::prelude::*;
use warcraft_api::AttackRange;

#[derive(Props, Clone, PartialEq)]
pub struct RangeRowModel {
    pub value: AttackRange,
}

impl From<&RangeRowView> for RangeRowModel {
    fn from(view: &RangeRowView) -> Self {
        let RangeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for RangeRowModel {
    type View = RangeRowView;
}

use super::view::UnitCategorySectionView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCategorySectionModel {
    pub category_kind: UnitKind,
}

impl From<&UnitCategorySectionView> for UnitCategorySectionModel {
    fn from(view: &UnitCategorySectionView) -> Self {
        let UnitCategorySectionView { category_kind } = view.clone();
        Self { category_kind }
    }
}

impl ddd::Model for UnitCategorySectionModel {
    type View = UnitCategorySectionView;
}

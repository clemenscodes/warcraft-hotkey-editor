use super::view::UnitCategorySectionView;
use dioxus::prelude::*;
use warcraft_api::UnitCatalogGroup;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCategorySectionModel {
    pub group: UnitCatalogGroup,
}

impl From<&UnitCategorySectionView> for UnitCategorySectionModel {
    fn from(view: &UnitCategorySectionView) -> Self {
        let UnitCategorySectionView { group } = view.clone();
        Self { group }
    }
}

impl ddd::Model for UnitCategorySectionModel {
    type View = UnitCategorySectionView;
}

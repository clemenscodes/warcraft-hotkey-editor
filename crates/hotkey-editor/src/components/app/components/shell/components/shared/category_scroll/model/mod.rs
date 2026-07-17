use super::view::CategoryScrollView;
use dioxus::prelude::*;
use warcraft_api::UnitCatalogGroup;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryScrollModel {
    pub groups: Vec<UnitCatalogGroup>,
}

impl From<&CategoryScrollView> for CategoryScrollModel {
    fn from(view: &CategoryScrollView) -> Self {
        let CategoryScrollView { groups } = view.clone();
        Self { groups }
    }
}

impl ddd::Model for CategoryScrollModel {
    type View = CategoryScrollView;
}

use super::view::CategoryTrackView;
use dioxus::prelude::*;
use warcraft_api::UnitCatalogGroup;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryTrackModel {
    pub groups: Vec<UnitCatalogGroup>,
}

impl From<&CategoryTrackView> for CategoryTrackModel {
    fn from(view: &CategoryTrackView) -> Self {
        let CategoryTrackView { groups } = view.clone();
        Self { groups }
    }
}

impl ddd::Model for CategoryTrackModel {
    type View = CategoryTrackView;
}

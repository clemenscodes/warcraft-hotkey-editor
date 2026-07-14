use super::view::CategoryScrollView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryScrollModel {
    pub sections: Vec<UnitKind>,
}

impl From<&CategoryScrollView> for CategoryScrollModel {
    fn from(view: &CategoryScrollView) -> Self {
        let CategoryScrollView { sections } = view.clone();
        Self { sections }
    }
}

impl ddd::Model for CategoryScrollModel {
    type View = CategoryScrollView;
}

use super::view::CategoryFilterView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryFilterModel {
    pub kind: UnitKind,
}

impl From<&CategoryFilterView> for CategoryFilterModel {
    fn from(view: &CategoryFilterView) -> Self {
        let CategoryFilterView { kind } = view.clone();
        Self { kind }
    }
}

impl ddd::Model for CategoryFilterModel {
    type View = CategoryFilterView;
}

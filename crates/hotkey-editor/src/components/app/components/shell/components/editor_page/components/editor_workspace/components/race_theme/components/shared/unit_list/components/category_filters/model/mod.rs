use super::view::CategoryFiltersView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryFiltersModel {
    pub tabs: Vec<UnitKind>,
}

impl From<&CategoryFiltersView> for CategoryFiltersModel {
    fn from(view: &CategoryFiltersView) -> Self {
        let CategoryFiltersView { tabs } = view.clone();
        Self { tabs }
    }
}

impl ddd::Model for CategoryFiltersModel {
    type View = CategoryFiltersView;
}

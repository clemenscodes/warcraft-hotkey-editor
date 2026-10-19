use super::view::CategoryHotkeysView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryHotkeysModel {
    pub category: SystemHotkeysCategory,
}

impl From<&CategoryHotkeysView> for CategoryHotkeysModel {
    fn from(view: &CategoryHotkeysView) -> Self {
        let category = view.category;
        Self { category }
    }
}

impl ddd::Layered for CategoryHotkeysModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CategoryHotkeysModel {
    type View = CategoryHotkeysView;
}

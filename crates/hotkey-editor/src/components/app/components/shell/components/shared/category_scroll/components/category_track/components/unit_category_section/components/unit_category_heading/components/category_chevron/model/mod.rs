use super::view::CategoryChevronView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryChevronModel {
    pub is_collapsed: bool,
}

impl From<&CategoryChevronView> for CategoryChevronModel {
    fn from(view: &CategoryChevronView) -> Self {
        let CategoryChevronView { is_collapsed } = view.clone();
        Self { is_collapsed }
    }
}

impl ddd::Layered for CategoryChevronModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CategoryChevronModel {
    type View = CategoryChevronView;
}

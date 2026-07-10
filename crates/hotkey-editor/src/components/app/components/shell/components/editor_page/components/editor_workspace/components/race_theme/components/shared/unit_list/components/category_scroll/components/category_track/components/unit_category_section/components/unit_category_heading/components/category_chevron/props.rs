use super::view::CategoryChevronView;
use dioxus::prelude::*;

/// Whether the category is collapsed, which points the chevron right vs down.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryChevronProps {
    pub is_collapsed: bool,
}

impl From<&CategoryChevronView> for CategoryChevronProps {
    fn from(view: &CategoryChevronView) -> Self {
        let CategoryChevronView { is_collapsed } = view.clone();
        Self { is_collapsed }
    }
}

impl ddd::Props for CategoryChevronProps {
    type View = CategoryChevronView;
}

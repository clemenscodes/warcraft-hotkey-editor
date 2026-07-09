use dioxus::prelude::*;

/// Whether the category is collapsed, which points the chevron right vs down.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryChevronProps {
    pub is_collapsed: bool,
}

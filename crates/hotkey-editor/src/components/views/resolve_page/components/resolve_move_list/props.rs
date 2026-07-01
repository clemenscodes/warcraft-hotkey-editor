use dioxus::prelude::*;

/// The auto-fill grid of move cards. `data_category` tags the active section.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveMoveListProps {
    pub data_category: &'static str,
    pub children: Element,
}

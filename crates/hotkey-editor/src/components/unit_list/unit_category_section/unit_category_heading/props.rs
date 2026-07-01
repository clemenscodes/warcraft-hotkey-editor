use dioxus::prelude::*;

/// A category heading: its label, kind attribute, collapsed state, and the toggle
/// handler.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategoryHeadingProps {
    #[props(into)]
    pub label: String,
    pub kind_attr: &'static str,
    pub is_collapsed: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

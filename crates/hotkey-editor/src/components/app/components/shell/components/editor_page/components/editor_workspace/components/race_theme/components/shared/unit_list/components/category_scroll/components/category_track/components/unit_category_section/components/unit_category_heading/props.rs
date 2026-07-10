use super::components::category_chevron::CategoryChevronProps;
use dioxus::prelude::*;

/// A category heading: its label, collapsed state, and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategoryHeadingProps {
    #[props(into)]
    pub label: String,
    pub is_collapsed: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&UnitCategoryHeadingProps> for CategoryChevronProps {
    fn from(props: &UnitCategoryHeadingProps) -> Self {
        let is_collapsed = props.is_collapsed;
        Self { is_collapsed }
    }
}

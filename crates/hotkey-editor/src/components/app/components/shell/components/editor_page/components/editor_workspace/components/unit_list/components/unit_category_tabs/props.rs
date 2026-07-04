use dioxus::prelude::*;

/// The tab row wraps the individual category tabs passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategoryTabsProps {
    pub children: Element,
}

use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// The tab row's props: the unit kinds to render as category tabs, in display order.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabsProps {
    pub tabs: Vec<UnitKind>,
}

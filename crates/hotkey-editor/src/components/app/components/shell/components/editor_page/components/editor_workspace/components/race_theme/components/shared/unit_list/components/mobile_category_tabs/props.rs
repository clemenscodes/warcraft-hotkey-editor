use super::view::MobileCategoryTabsView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// The tab row's props: the unit kinds to render as category tabs, in display order.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabsProps {
    pub tabs: Vec<UnitKind>,
}

impl From<&MobileCategoryTabsView> for MobileCategoryTabsProps {
    fn from(view: &MobileCategoryTabsView) -> Self {
        let MobileCategoryTabsView { tabs } = view.clone();
        Self { tabs }
    }
}

impl ddd::Props for MobileCategoryTabsProps {
    type View = MobileCategoryTabsView;
}

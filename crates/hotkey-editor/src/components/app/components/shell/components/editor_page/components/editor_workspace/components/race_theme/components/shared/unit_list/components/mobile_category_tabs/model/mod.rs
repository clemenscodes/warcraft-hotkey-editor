use super::view::MobileCategoryTabsView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabsModel {
    pub tabs: Vec<UnitKind>,
}

impl From<&MobileCategoryTabsView> for MobileCategoryTabsModel {
    fn from(view: &MobileCategoryTabsView) -> Self {
        let MobileCategoryTabsView { tabs } = view.clone();
        Self { tabs }
    }
}

impl ddd::Model for MobileCategoryTabsModel {
    type View = MobileCategoryTabsView;
}

use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;
use super::view::ClosedBreadcrumbsMenuView;
use dioxus::prelude::*;

/// The closed menu's input: one tab descriptor per category, each carrying
/// `menu_open = false` so the tabs render their tab-bar look. Carrying domain
/// descriptors as data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuModel {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl From<&ClosedBreadcrumbsMenuView> for ClosedBreadcrumbsMenuModel {
    fn from(view: &ClosedBreadcrumbsMenuView) -> Self {
        let ClosedBreadcrumbsMenuView { tabs } = view.clone();
        Self { tabs }
    }
}

impl ddd::Model for ClosedBreadcrumbsMenuModel {
    type View = ClosedBreadcrumbsMenuView;
}

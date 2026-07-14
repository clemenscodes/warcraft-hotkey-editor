use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;
use super::view::ClosedBreadcrumbsMenuView;
use dioxus::prelude::*;

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

use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;
use super::view::OpenBreadcrumbsMenuView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuModel {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl From<&OpenBreadcrumbsMenuView> for OpenBreadcrumbsMenuModel {
    fn from(view: &OpenBreadcrumbsMenuView) -> Self {
        let OpenBreadcrumbsMenuView { tabs } = view.clone();
        Self { tabs }
    }
}

impl ddd::Model for OpenBreadcrumbsMenuModel {
    type View = OpenBreadcrumbsMenuView;
}

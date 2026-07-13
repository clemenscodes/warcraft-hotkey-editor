use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;
use super::view::OpenBreadcrumbsMenuView;
use dioxus::prelude::*;

/// The open menu's input: one tab descriptor per category, each carrying
/// `menu_open = true` so the tabs render their popover look. Carrying domain
/// descriptors as data is passing data, not `Element`.
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

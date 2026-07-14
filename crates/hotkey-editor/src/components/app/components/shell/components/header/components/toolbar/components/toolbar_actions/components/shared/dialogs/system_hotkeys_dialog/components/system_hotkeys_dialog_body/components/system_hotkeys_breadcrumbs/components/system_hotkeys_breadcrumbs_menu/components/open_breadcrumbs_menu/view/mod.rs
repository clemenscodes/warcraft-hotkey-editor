use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

#[derive(Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for OpenBreadcrumbsMenuView {}

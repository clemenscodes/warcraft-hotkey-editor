use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

#[derive(Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for ClosedBreadcrumbsMenuView {}

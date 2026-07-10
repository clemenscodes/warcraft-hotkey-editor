use super::super::super::hooks::SystemHotkeysCategoryTabDescriptor;

/// The published `View` contract mirroring [`OpenBreadcrumbsMenuProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for OpenBreadcrumbsMenuView {}

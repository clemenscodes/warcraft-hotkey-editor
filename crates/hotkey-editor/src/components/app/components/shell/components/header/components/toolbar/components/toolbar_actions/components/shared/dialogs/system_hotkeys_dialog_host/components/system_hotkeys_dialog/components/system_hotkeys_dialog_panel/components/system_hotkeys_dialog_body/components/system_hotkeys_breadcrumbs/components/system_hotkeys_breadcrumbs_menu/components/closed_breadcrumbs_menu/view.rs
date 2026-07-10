use super::super::super::hooks::SystemHotkeysCategoryTabDescriptor;

/// The published `View` contract mirroring [`ClosedBreadcrumbsMenuProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for ClosedBreadcrumbsMenuView {}

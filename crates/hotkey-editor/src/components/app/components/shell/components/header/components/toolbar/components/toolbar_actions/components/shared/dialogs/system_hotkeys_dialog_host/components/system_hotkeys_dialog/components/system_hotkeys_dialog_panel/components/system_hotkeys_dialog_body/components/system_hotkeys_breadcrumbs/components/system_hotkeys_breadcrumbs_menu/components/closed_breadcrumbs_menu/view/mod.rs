use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

/// The published `View` contract mirroring [`ClosedBreadcrumbsMenuModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for ClosedBreadcrumbsMenuView {}

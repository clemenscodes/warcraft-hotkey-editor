use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

/// The published `View` contract mirroring [`OpenBreadcrumbsMenuModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::View for OpenBreadcrumbsMenuView {}

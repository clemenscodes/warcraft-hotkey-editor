use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

#[derive(Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::Layered for OpenBreadcrumbsMenuView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for OpenBreadcrumbsMenuView {}

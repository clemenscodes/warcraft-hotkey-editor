use super::super::super::presentation::SystemHotkeysCategoryTabDescriptor;

#[derive(Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuView {
    pub(crate) tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}

impl ddd::Layered for ClosedBreadcrumbsMenuView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ClosedBreadcrumbsMenuView {}

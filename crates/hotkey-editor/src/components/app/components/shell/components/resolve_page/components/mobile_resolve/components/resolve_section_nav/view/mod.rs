use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;

#[derive(Clone, PartialEq)]
pub struct ResolveSectionNavView {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl ddd::View for ResolveSectionNavView {}

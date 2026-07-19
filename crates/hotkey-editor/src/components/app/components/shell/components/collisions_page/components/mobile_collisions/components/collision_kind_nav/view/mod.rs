use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;

#[derive(Clone, PartialEq)]
pub struct CollisionKindNavView {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl ddd::View for CollisionKindNavView {}

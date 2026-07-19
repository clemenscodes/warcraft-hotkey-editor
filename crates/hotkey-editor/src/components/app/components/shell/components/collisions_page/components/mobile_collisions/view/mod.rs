use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;

#[derive(Clone, PartialEq)]
pub struct MobileCollisionsView {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub content: ContentModel,
}

impl ddd::View for MobileCollisionsView {}

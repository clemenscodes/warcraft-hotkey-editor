use super::view::MobileCollisionsView;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MobileCollisionsModel {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub content: ContentModel,
}

impl From<&MobileCollisionsView> for MobileCollisionsModel {
    fn from(view: &MobileCollisionsView) -> Self {
        let MobileCollisionsView {
            breadcrumbs,
            content,
        } = view.clone();
        Self {
            breadcrumbs,
            content,
        }
    }
}

impl ddd::Model for MobileCollisionsModel {
    type View = MobileCollisionsView;
}

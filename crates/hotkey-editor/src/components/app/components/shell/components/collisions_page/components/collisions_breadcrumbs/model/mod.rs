use super::view::CollisionsBreadcrumbsView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

/// The collisions page's breadcrumb bar region: the prepared collision-kind tabs the page
/// hands it. The bar's assistive-tech name is fixed ("Collision categories"), so it is the
/// region's own identity, not a field.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsBreadcrumbsModel {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl From<&CollisionsBreadcrumbsView> for CollisionsBreadcrumbsModel {
    fn from(view: &CollisionsBreadcrumbsView) -> Self {
        let CollisionsBreadcrumbsView { breadcrumbs } = view.clone();
        Self { breadcrumbs }
    }
}

impl ddd::Model for CollisionsBreadcrumbsModel {
    type View = CollisionsBreadcrumbsView;
}

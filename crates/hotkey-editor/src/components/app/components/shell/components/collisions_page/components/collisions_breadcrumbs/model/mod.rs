use super::view::CollisionsBreadcrumbsView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

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

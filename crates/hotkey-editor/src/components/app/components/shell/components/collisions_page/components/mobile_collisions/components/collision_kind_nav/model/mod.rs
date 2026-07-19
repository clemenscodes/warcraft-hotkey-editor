use super::view::CollisionKindNavView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionKindNavModel {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl From<&CollisionKindNavView> for CollisionKindNavModel {
    fn from(view: &CollisionKindNavView) -> Self {
        let CollisionKindNavView { breadcrumbs } = view.clone();
        Self { breadcrumbs }
    }
}

impl ddd::Model for CollisionKindNavModel {
    type View = CollisionKindNavView;
}

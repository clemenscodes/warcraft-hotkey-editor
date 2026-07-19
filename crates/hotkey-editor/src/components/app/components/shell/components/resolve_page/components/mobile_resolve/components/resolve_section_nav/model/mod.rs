use super::view::ResolveSectionNavView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveSectionNavModel {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl From<&ResolveSectionNavView> for ResolveSectionNavModel {
    fn from(view: &ResolveSectionNavView) -> Self {
        let ResolveSectionNavView { breadcrumbs } = view.clone();
        Self { breadcrumbs }
    }
}

impl ddd::Model for ResolveSectionNavModel {
    type View = ResolveSectionNavView;
}

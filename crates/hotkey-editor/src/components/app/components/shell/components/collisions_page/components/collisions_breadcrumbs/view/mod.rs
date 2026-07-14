use super::CollisionsBreadcrumbs;
use super::model::CollisionsBreadcrumbsModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct CollisionsBreadcrumbsView {
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl ddd::View for CollisionsBreadcrumbsView {}

impl Render for CollisionsBreadcrumbsView {
    type Model = CollisionsBreadcrumbsModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let breadcrumbs = self.breadcrumbs.clone();
        rsx! {
            CollisionsBreadcrumbs {
                breadcrumbs,
            }
        }
    }
}

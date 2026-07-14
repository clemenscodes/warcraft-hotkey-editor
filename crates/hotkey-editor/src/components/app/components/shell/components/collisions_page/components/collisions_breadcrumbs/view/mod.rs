use super::CollisionsBreadcrumbs;
use super::model::CollisionsBreadcrumbsModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` for the collisions breadcrumb region: the prepared collision-kind
/// tabs. It is also the collisions page frame's header region: it `impl Render` and renders
/// the `CollisionsBreadcrumbs` once, so the page places the published `View` directly, with
/// no ad-hoc region type.
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

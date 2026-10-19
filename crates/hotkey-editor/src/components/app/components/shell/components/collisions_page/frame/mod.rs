use super::components::body::BodyView;
use super::components::collisions_breadcrumbs::CollisionsBreadcrumbsView;
use browser_kit::frame::Frame;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

#[derive(Clone, PartialEq, Default)]
pub struct CollisionsPageFrame {
    pub(super) header: CollisionsBreadcrumbsView,
    pub(super) body: BodyView,
}

impl Frame for CollisionsPageFrame {
    type Output = Element;
    type Header = CollisionsBreadcrumbsView;
    type Body = BodyView;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }

    fn header(&self) -> Option<Self::Header> {
        let header = self.header.clone();
        Some(header)
    }
}

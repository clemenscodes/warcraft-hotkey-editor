use super::components::body::BodyView;
use super::components::collisions_breadcrumbs::CollisionsBreadcrumbsView;
use browser_kit::frame::Frame;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

/// The collisions page's frame: the collision-kind breadcrumb header region above the
/// two-pane content body region. The collisions page builds this and hands it to the headless
/// `Page`, which places the regions inside the styled page container. The page owns no footer —
/// the shell owns the app footer — so that region defaults to `Empty`.
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

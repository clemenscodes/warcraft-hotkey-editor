use super::components::plan_body::PlanBodyView;
use super::components::resolve_plan_header::ResolvePlanHeaderView;
use browser_kit::frame::Frame;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

/// The resolve page's frame: the plan summary + breadcrumb header region above the scrollable
/// plan body region. The resolve page builds this for the plan state and hands it to the
/// headless `Page`, which places the regions inside the styled page container. The page owns
/// no footer — the shell owns the app footer — so that region defaults to `Empty`.
#[derive(Clone, PartialEq, Default)]
pub struct ResolvePageFrame {
    pub(super) header: ResolvePlanHeaderView,
    pub(super) body: PlanBodyView,
}

impl Frame for ResolvePageFrame {
    type Output = Element;
    type Header = ResolvePlanHeaderView;
    type Body = PlanBodyView;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }

    fn header(&self) -> Option<Self::Header> {
        let header = self.header.clone();
        Some(header)
    }
}

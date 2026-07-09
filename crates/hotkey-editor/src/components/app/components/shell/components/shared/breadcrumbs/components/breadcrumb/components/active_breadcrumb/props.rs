use dioxus::prelude::*;

/// The active breadcrumb tab's props: its label, live count, e2e slug, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub data_breadcrumb: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

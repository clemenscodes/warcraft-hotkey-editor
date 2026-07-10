use dioxus::prelude::*;

/// The active breadcrumb tab's props: its label, live count, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

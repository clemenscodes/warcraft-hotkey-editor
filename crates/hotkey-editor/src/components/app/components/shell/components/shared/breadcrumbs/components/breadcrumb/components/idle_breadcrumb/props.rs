use dioxus::prelude::*;

/// The idle breadcrumb tab's props: its label, live count, e2e slug, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub data_breadcrumb: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

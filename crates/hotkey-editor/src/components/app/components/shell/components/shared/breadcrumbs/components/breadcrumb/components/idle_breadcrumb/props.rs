use dioxus::prelude::*;

/// The idle breadcrumb tab's props: its label, live count, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

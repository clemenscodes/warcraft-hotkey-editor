use dioxus::prelude::*;

/// One move-category breadcrumb: its title, count, e2e slug, active flag, and the
/// handler selecting its section.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(into)]
    pub title: String,
    pub count: usize,
    pub data_breadcrumb: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

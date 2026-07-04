use super::components::breadcrumb::BreadcrumbProps;
use dioxus::prelude::*;

/// The move-category breadcrumb bar: one prepared tab per section (built in the
/// page hook so the select handlers close over the selection signal).
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbsProps {
    pub breadcrumbs: Vec<BreadcrumbProps>,
}

use super::components::breadcrumb::BreadcrumbProps;
use dioxus::prelude::*;

/// A breadcrumb bar: one prepared tab per entry, separated by "|". The tabs are
/// built by the page that owns the data (each tab closes over its own navigation
/// handler), so this bar is purely presentational. `aria_label` names the bar for
/// assistive tech ("Collision categories", "Move categories", …).
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbsProps {
    pub breadcrumbs: Vec<BreadcrumbProps>,
    pub aria_label: &'static str,
}

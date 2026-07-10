use super::breadcrumb_view::BreadcrumbView;
use dioxus::prelude::*;

/// A breadcrumb bar: one prepared tab per entry, separated by "|". The tabs are
/// built by the page that owns the data (each tab closes over its own navigation
/// handler), so this bar is purely presentational. `aria_label` names the bar for
/// assistive tech ("Collision categories", "Move categories", …).
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbsProps {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub aria_label: &'static str,
}

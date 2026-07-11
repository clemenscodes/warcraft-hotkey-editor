use super::breadcrumb_view::BreadcrumbView;
use super::view::BreadcrumbsView;
use dioxus::prelude::*;

/// A breadcrumb bar: one prepared tab per entry, separated by "|". The tabs are
/// built by the page that owns the data (each tab closes over its own navigation
/// handler), so this bar is purely presentational. `aria_label` names the bar for
/// assistive tech ("Collision categories", "Move categories", …).
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbsModel {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub aria_label: &'static str,
}

impl From<&BreadcrumbsView> for BreadcrumbsModel {
    fn from(view: &BreadcrumbsView) -> Self {
        let BreadcrumbsView {
            breadcrumbs,
            aria_label,
        } = view.clone();
        Self {
            breadcrumbs,
            aria_label,
        }
    }
}

impl ddd::Model for BreadcrumbsModel {
    type View = BreadcrumbsView;
}

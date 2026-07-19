use super::BreadcrumbView;
use super::view::BreadcrumbsView;
use dioxus::prelude::*;

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

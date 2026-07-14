use super::breadcrumb_view::BreadcrumbView;

#[derive(Clone, PartialEq)]
pub struct BreadcrumbsView {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub aria_label: &'static str,
}

impl ddd::View for BreadcrumbsView {}

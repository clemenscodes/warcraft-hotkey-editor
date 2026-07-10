use super::breadcrumb_view::BreadcrumbView;

/// The published `View` contract mirroring [`BreadcrumbsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbsView {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub aria_label: &'static str,
}

impl ddd::View for BreadcrumbsView {}

use super::BreadcrumbView;

#[derive(Clone, PartialEq)]
pub struct BreadcrumbsView {
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub aria_label: &'static str,
}

impl ddd::Layered for BreadcrumbsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BreadcrumbsView {}

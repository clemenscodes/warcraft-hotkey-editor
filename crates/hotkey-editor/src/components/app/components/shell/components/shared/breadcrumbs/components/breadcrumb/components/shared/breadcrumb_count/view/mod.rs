#[derive(Clone, PartialEq)]
pub struct BreadcrumbCountView {
    pub count: usize,
}

impl ddd::Layered for BreadcrumbCountView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BreadcrumbCountView {}

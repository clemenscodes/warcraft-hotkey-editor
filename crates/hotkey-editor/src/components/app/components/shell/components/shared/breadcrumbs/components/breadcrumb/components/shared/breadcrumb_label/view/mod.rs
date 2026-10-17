#[derive(Clone, PartialEq)]
pub struct BreadcrumbLabelView {
    pub text: String,
}

impl ddd::Layered for BreadcrumbLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BreadcrumbLabelView {}

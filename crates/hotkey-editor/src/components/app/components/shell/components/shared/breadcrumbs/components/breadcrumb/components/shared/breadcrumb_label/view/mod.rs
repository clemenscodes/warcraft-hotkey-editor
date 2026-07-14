#[derive(Clone, PartialEq)]
pub struct BreadcrumbLabelView {
    pub text: String,
}

impl ddd::View for BreadcrumbLabelView {}

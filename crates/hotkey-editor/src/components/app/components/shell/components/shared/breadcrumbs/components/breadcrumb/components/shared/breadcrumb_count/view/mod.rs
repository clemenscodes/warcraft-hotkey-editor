#[derive(Clone, PartialEq)]
pub struct BreadcrumbCountView {
    pub count: usize,
}

impl ddd::View for BreadcrumbCountView {}

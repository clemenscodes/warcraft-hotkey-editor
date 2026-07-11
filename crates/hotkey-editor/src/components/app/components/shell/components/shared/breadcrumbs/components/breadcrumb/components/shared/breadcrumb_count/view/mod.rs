/// The published `View` contract mirroring [`BreadcrumbCountModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbCountView {
    pub count: usize,
}

impl ddd::View for BreadcrumbCountView {}

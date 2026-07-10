/// The published `View` contract mirroring [`BreadcrumbLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbLabelView {
    pub text: String,
}

impl ddd::View for BreadcrumbLabelView {}

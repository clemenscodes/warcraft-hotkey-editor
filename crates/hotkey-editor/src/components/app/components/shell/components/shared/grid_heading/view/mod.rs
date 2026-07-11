/// The published `View` contract mirroring [`GridHeadingModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridHeadingView {
    pub heading: &'static str,
}

impl ddd::View for GridHeadingView {}

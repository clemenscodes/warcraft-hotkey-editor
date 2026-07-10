/// The published `View` contract mirroring [`GridHeadingProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridHeadingView {
    pub heading: &'static str,
}

impl ddd::View for GridHeadingView {}

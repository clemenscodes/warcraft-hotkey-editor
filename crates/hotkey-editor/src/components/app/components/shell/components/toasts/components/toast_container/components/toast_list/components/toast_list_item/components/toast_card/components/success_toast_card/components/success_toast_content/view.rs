/// The published `View` contract mirroring [`SuccessToastContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SuccessToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for SuccessToastContentView {}

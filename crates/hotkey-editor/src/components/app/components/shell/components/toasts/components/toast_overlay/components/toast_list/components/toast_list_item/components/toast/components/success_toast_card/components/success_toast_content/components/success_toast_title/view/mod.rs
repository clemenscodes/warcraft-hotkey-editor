/// The published `View` contract mirroring [`SuccessToastTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SuccessToastTitleView {
    pub title: String,
}

impl ddd::View for SuccessToastTitleView {}

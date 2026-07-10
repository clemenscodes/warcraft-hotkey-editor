/// The published `View` contract mirroring [`SuccessToastTitleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SuccessToastTitleView {
    pub title: String,
}

impl ddd::View for SuccessToastTitleView {}

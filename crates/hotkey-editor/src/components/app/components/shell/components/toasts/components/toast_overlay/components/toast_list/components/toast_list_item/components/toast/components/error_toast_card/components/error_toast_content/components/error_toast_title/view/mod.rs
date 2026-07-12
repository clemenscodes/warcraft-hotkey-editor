/// The published `View` contract mirroring [`ErrorToastTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ErrorToastTitleView {
    pub title: String,
}

impl ddd::View for ErrorToastTitleView {}

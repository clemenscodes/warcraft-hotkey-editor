/// The published `View` contract mirroring [`ToastTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastTitleView {
    pub title: String,
}

impl ddd::View for ToastTitleView {}

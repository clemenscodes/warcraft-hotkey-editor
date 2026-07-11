/// The published `View` contract mirroring [`InfoToastTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoToastTitleView {
    pub title: String,
}

impl ddd::View for InfoToastTitleView {}

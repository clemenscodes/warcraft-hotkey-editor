/// The published `View` contract mirroring [`WarningToastTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct WarningToastTitleView {
    pub title: String,
}

impl ddd::View for WarningToastTitleView {}

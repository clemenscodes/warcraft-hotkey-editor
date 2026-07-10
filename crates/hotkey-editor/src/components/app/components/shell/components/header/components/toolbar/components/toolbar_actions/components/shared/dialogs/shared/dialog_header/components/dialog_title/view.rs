/// The published `View` contract mirroring [`DialogTitleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DialogTitleView {
    pub title: String,
}

impl ddd::View for DialogTitleView {}

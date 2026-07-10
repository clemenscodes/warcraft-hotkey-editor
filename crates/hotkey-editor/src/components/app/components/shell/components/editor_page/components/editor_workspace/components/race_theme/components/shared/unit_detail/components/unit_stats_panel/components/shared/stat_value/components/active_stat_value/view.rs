/// The published `View` contract mirroring [`ActiveStatValueProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveStatValueView {
    pub text: String,
}

impl ddd::View for ActiveStatValueView {}

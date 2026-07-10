/// The published `View` contract mirroring [`ClearLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ClearLabelView {
    pub text: String,
}

impl ddd::View for ClearLabelView {}

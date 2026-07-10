/// The published `View` contract mirroring [`RegenLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegenLabelView {
    pub text: String,
}

impl ddd::View for RegenLabelView {}

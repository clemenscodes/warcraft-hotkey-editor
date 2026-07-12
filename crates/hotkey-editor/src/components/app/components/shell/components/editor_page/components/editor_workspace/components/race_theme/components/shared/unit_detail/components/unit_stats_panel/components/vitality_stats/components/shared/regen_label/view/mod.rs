/// The published `View` contract mirroring [`RegenLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegenLabelView {
    pub text: String,
}

impl ddd::View for RegenLabelView {}

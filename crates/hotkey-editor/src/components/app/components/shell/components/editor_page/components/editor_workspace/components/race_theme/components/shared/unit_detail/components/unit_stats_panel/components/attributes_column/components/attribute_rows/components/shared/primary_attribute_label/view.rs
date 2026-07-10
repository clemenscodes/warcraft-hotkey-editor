/// The published `View` contract mirroring [`PrimaryAttributeLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PrimaryAttributeLabelView {
    pub text: String,
}

impl ddd::View for PrimaryAttributeLabelView {}

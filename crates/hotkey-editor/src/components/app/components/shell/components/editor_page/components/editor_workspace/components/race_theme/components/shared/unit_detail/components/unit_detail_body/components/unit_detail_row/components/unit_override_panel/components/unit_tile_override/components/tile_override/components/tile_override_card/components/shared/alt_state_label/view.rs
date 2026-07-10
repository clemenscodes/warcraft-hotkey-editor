/// The published `View` contract mirroring [`AltStateLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateLabelView {
    pub text: Option<String>,
}

impl ddd::View for AltStateLabelView {}

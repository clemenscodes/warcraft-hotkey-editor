/// The published `View` contract mirroring [`AltStateLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateLabelView {
    pub text: Option<String>,
}

impl ddd::View for AltStateLabelView {}

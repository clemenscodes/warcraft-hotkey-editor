/// The published `View` contract mirroring [`AltStateLineProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateLineView {
    pub text: String,
}

impl ddd::View for AltStateLineView {}

/// The published `View` contract mirroring [`AltStateHeaderLabelColumnProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnView {
    pub text: Option<String>,
}

impl ddd::View for AltStateHeaderLabelColumnView {}

/// The published `View` contract mirroring [`UnitNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitNameView {
    pub text: &'static str,
}

impl ddd::View for UnitNameView {}

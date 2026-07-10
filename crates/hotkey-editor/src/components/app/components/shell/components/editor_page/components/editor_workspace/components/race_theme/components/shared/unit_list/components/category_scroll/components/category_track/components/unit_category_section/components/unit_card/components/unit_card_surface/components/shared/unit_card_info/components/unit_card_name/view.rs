/// The published `View` contract mirroring [`UnitCardNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardNameView {
    pub text: String,
}

impl ddd::View for UnitCardNameView {}

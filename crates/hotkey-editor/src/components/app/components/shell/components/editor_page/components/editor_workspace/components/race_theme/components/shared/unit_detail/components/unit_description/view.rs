/// The published `View` contract mirroring [`UnitDescriptionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDescriptionView {
    pub text: String,
}

impl ddd::View for UnitDescriptionView {}

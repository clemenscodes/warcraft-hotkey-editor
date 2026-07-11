/// The published `View` contract mirroring [`AltPositionPickerExplainerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltPositionPickerExplainerView {
    pub text: String,
}

impl ddd::View for AltPositionPickerExplainerView {}

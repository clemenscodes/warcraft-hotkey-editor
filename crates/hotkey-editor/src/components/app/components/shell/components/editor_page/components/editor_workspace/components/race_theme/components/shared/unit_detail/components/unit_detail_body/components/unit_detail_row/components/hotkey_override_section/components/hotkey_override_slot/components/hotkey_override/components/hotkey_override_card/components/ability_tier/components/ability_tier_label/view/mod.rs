/// The published `View` contract mirroring [`AbilityTierLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityTierLabelView {
    pub text: String,
}

impl ddd::View for AbilityTierLabelView {}

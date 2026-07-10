/// The published `View` contract mirroring [`AbilityFillProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityFillView {
    pub active: bool,
}

impl ddd::View for AbilityFillView {}

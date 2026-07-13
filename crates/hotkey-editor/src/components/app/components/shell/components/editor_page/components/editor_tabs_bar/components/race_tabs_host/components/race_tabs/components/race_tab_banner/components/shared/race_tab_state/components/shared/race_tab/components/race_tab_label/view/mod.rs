/// The published `View` contract mirroring [`RaceTabLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RaceTabLabelView {
    pub label: String,
}

impl ddd::View for RaceTabLabelView {}

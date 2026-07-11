/// The published `View` contract mirroring [`TileLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileLabelView {
    /// The label text, present only when the occupant has no icon.
    pub text: Option<String>,
}

impl ddd::View for TileLabelView {}

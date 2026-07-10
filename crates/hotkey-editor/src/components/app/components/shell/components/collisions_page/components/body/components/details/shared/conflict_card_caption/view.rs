/// The published `View` contract mirroring [`ConflictCardCaptionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictCardCaptionView {
    pub text: String,
}

impl ddd::View for ConflictCardCaptionView {}

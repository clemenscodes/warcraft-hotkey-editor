/// The published `View` contract mirroring [`MoveNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveNameView {
    pub text: String,
    pub is_link: bool,
}

impl ddd::View for MoveNameView {}

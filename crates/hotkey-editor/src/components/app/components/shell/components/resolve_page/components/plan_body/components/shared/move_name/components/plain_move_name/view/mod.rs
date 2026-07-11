/// The published `View` contract mirroring [`PlainMoveNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlainMoveNameView {
    pub text: String,
}

impl ddd::View for PlainMoveNameView {}

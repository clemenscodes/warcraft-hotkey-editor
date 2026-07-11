/// The published `View` contract mirroring [`LinkMoveNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LinkMoveNameView {
    pub text: String,
}

impl ddd::View for LinkMoveNameView {}

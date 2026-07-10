/// The published `View` contract mirroring [`EmptyMessageProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EmptyMessageView {
    pub text: String,
}

impl ddd::View for EmptyMessageView {}

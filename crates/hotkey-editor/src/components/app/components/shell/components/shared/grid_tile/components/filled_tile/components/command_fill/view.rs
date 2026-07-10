/// The published `View` contract mirroring [`CommandFillProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CommandFillView {
    pub active: bool,
}

impl ddd::View for CommandFillView {}

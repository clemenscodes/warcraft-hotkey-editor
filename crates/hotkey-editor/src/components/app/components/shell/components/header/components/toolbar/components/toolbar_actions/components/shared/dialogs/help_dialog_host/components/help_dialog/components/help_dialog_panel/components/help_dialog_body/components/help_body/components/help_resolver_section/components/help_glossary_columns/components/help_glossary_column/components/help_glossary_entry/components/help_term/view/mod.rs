/// The published `View` contract mirroring [`HelpTermModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpTermView {
    pub term: String,
}

impl ddd::View for HelpTermView {}

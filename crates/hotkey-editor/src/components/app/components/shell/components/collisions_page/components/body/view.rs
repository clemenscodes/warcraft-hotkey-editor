use super::model::ContentModel;

/// The published `View` contract mirroring [`BodyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BodyView {
    pub content: ContentModel,
}

impl ddd::View for BodyView {}

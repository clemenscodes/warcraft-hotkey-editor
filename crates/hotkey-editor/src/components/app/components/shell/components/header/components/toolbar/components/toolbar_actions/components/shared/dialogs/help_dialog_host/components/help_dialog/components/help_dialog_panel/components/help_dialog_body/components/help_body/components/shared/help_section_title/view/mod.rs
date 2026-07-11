/// The published `View` contract mirroring [`HelpSectionTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpSectionTitleView {
    pub title: String,
}

impl ddd::View for HelpSectionTitleView {}

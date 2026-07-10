/// The published `View` contract mirroring [`HelpSectionTitleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpSectionTitleView {
    pub title: String,
}

impl ddd::View for HelpSectionTitleView {}

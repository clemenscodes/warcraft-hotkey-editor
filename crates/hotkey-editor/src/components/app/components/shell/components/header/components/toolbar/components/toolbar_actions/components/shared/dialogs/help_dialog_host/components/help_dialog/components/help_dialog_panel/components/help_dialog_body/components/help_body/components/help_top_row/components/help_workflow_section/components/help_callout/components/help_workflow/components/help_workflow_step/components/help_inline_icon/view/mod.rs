/// The published `View` contract mirroring [`HelpInlineIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpInlineIconView {
    pub icon: &'static str,
}

impl ddd::View for HelpInlineIconView {}

/// The published `View` contract mirroring [`BelowLeftTooltipModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BelowLeftTooltipView {
    pub text: String,
}

impl ddd::View for BelowLeftTooltipView {}

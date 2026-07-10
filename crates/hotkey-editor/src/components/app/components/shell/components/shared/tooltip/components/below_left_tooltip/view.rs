/// The published `View` contract mirroring [`BelowLeftTooltipProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BelowLeftTooltipView {
    pub text: String,
}

impl ddd::View for BelowLeftTooltipView {}

/// The published `View` contract mirroring [`AboveCenterTooltipProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AboveCenterTooltipView {
    pub text: String,
}

impl ddd::View for AboveCenterTooltipView {}

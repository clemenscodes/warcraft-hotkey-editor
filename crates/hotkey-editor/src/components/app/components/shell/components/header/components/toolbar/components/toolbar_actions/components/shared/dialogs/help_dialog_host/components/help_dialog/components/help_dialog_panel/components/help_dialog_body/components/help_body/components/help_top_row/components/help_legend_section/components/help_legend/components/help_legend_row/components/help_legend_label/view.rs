/// The published `View` contract mirroring [`HelpLegendLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpLegendLabelView {
    pub label: String,
}

impl ddd::View for HelpLegendLabelView {}

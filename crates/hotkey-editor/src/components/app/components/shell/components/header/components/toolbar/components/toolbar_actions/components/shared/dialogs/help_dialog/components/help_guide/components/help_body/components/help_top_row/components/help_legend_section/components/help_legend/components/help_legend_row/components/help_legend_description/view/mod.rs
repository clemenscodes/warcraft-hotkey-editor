/// The published `View` contract mirroring [`HelpLegendDescriptionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpLegendDescriptionView {
    pub description: String,
}

impl ddd::View for HelpLegendDescriptionView {}

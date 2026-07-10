/// The published `View` contract mirroring [`HelpLegendIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpLegendIconView {
    pub icon: &'static str,
}

impl ddd::View for HelpLegendIconView {}

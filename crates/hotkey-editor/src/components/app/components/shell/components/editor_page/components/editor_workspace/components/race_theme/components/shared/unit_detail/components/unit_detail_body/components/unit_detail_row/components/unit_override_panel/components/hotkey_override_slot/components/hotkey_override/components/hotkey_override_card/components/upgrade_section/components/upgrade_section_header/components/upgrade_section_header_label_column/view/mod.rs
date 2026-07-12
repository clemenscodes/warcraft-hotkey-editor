/// The published `View` contract mirroring [`UpgradeSectionHeaderLabelColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UpgradeSectionHeaderLabelColumnView {
    pub text: Option<String>,
}

impl ddd::View for UpgradeSectionHeaderLabelColumnView {}

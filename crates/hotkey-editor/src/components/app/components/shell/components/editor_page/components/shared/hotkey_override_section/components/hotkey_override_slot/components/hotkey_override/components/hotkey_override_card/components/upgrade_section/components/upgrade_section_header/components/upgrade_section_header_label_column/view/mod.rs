#[derive(Clone, PartialEq)]
pub struct UpgradeSectionHeaderLabelColumnView {
    pub text: Option<String>,
}

impl ddd::Layered for UpgradeSectionHeaderLabelColumnView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UpgradeSectionHeaderLabelColumnView {}

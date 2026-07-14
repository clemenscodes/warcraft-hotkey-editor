use super::view::UpgradeSectionHeaderLabelColumnView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderLabelColumnModel {
    pub text: Option<String>,
}

impl From<&UpgradeSectionHeaderLabelColumnView> for UpgradeSectionHeaderLabelColumnModel {
    fn from(view: &UpgradeSectionHeaderLabelColumnView) -> Self {
        let UpgradeSectionHeaderLabelColumnView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for UpgradeSectionHeaderLabelColumnModel {
    type View = UpgradeSectionHeaderLabelColumnView;
}

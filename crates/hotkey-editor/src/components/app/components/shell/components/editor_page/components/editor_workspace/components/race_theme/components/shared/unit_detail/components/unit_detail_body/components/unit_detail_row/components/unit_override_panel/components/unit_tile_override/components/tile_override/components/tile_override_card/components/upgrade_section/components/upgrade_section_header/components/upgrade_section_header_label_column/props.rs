use super::view::UpgradeSectionHeaderLabelColumnView;
use dioxus::prelude::*;

/// The label column of the upgraded-form header row.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderLabelColumnProps {
    pub text: Option<String>,
}

impl From<&UpgradeSectionHeaderLabelColumnView> for UpgradeSectionHeaderLabelColumnProps {
    fn from(view: &UpgradeSectionHeaderLabelColumnView) -> Self {
        let UpgradeSectionHeaderLabelColumnView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for UpgradeSectionHeaderLabelColumnProps {
    type View = UpgradeSectionHeaderLabelColumnView;
}

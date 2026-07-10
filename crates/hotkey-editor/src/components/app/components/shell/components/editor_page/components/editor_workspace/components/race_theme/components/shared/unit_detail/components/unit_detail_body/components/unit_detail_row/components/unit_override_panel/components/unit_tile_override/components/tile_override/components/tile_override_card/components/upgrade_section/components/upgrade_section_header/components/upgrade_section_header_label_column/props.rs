use dioxus::prelude::*;

/// The label column of the upgraded-form header row.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderLabelColumnProps {
    pub text: Option<String>,
}

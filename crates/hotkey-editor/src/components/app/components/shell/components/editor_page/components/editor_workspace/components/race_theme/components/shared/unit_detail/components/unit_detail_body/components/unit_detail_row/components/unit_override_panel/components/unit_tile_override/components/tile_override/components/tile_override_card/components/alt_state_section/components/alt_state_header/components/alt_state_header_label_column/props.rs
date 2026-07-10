use dioxus::prelude::*;

/// The label column of the off-state header row.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnProps {
    pub text: Option<String>,
}

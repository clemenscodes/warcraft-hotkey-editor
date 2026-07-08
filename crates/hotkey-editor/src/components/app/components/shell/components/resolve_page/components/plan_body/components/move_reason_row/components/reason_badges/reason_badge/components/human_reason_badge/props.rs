use dioxus::prelude::*;

/// The badge's label text.
#[derive(Props, Clone, PartialEq)]
pub struct HumanReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

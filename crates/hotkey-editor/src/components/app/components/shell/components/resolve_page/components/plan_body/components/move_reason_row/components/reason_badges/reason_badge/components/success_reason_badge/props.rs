use dioxus::prelude::*;

/// The badge's label text.
#[derive(Props, Clone, PartialEq)]
pub struct SuccessReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

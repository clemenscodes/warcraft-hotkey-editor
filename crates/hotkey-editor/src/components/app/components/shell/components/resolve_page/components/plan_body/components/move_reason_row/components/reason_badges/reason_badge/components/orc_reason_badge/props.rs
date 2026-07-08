use dioxus::prelude::*;

/// The badge's label text.
#[derive(Props, Clone, PartialEq)]
pub struct OrcReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

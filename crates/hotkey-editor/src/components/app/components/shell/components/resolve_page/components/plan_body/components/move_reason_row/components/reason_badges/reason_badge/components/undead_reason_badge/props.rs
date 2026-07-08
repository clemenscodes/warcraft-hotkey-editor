use dioxus::prelude::*;

/// The badge's label text.
#[derive(Props, Clone, PartialEq)]
pub struct UndeadReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

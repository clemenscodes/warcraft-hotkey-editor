use dioxus::prelude::*;

/// The alt-state header row wraps its label text and controls (children).
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderProps {
    pub children: Element,
}

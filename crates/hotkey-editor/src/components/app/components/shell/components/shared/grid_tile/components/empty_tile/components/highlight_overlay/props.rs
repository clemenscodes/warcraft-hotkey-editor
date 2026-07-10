use dioxus::prelude::*;

/// Mounts only on the one coordinate a mini grid marks; every other empty slot leaves
/// `active` false and early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct HighlightOverlayProps {
    pub active: bool,
}

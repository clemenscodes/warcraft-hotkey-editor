use dioxus::prelude::*;

/// Mounts only when a drop onto this empty slot is refused (another ability's off-state
/// reserves it); every other empty slot leaves `active` false and early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct BlockedDropTargetOverlayProps {
    pub active: bool,
}

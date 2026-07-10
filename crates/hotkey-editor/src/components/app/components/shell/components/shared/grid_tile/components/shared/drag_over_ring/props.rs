use dioxus::prelude::*;

/// Mounts only on the tile the cursor hovers during a drag; every other tile leaves
/// `active` false and early-returns, so its presence is the under-cursor signal the tile
/// root's gold border keys off. Shared by the filled and empty tiles, which each set
/// `active` from their own drag-over flag.
#[derive(Props, Clone, PartialEq)]
pub struct DragOverRingProps {
    pub active: bool,
}

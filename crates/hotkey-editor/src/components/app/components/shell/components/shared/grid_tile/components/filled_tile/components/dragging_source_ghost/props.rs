use dioxus::prelude::*;

/// Mounts only for the tile currently lifted as a drag's source; every other tile
/// leaves `active` false and early-returns, so its mere presence is the tile's
/// dragging-source signal — the `FilledTile` root's own border keys off it, and the
/// drag mechanics never read it.
#[derive(Props, Clone, PartialEq)]
pub struct DraggingSourceGhostProps {
    pub active: bool,
}

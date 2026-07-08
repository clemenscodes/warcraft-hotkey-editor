/// Which look an empty tile draws. Mutually exclusive and expressed as a
/// conditionally-mounted overlay child (`DropTargetOverlay` / `BlockedDropTargetOverlay`
/// / `HighlightOverlay`), never a class swap: an idle empty slot mounts none, and the
/// tile root reacts to whichever overlay is present. This is only the *selector* for
/// the overlay props; the root stays one mounted element across every value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EmptyTileState {
    #[default]
    Empty,
    DropTarget,
    BlockedDropTarget,
    Highlighted,
}

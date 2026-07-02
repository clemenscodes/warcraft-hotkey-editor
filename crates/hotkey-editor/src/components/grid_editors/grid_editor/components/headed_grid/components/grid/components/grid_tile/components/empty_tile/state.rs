/// The styled state of an empty tile. Mutually exclusive: an idle empty slot, the
/// current drop-target candidate during a drag, a slot a drop is refused on, or the
/// one coordinate a mini grid marks.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EmptyTileState {
    #[default]
    Empty,
    DropTarget,
    BlockedDropTarget,
    Highlighted,
}

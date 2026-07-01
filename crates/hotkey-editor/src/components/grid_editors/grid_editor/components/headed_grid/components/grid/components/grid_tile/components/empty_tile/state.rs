/// The styled state of an empty tile. Mutually exclusive: an idle empty slot, the
/// current drop-target candidate during a drag, or a slot a drop is refused on.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EmptyTileState {
    #[default]
    Empty,
    DropTarget,
    BlockedDropTarget,
}

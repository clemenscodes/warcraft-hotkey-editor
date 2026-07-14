#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GridTileState {
    #[default]
    Empty,
    Filled,
    Command,
    Selected,
    DropTarget,
    BlockedDropTarget,
    Highlighted,
}

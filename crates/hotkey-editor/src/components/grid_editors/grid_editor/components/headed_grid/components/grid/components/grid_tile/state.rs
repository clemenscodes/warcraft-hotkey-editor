/// The visual state of a tile, decided by the grid (which ability occupies it,
/// whether it is selected, whether a drag is targeting it). The tile only
/// renders the look for the state it is given; it computes none of this itself.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GridTileState {
    /// No ability; idle.
    #[default]
    Empty,
    /// Holds an ability.
    Filled,
    /// Holds a built-in command (build, cancel, …).
    Command,
    /// Holds the currently selected ability.
    Selected,
    /// Empty and the current drop target candidate during a drag.
    DropTarget,
    /// Empty but reserved by another ability's off-state, so drops are refused.
    BlockedDropTarget,
}

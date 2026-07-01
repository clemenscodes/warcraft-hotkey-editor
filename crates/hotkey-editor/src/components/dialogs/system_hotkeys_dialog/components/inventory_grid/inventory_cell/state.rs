/// An inventory cell's glow state: idle, actively highlighted (its picker is open,
/// or it is the current drop target), or in a binding conflict. Whether the cell is
/// the drag source (which hides its contents) is a separate flag, since it can
/// combine with any of these. Chosen in the hook; the cell only renders the look.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum InventoryCellState {
    #[default]
    Idle,
    Active,
    Conflict,
}

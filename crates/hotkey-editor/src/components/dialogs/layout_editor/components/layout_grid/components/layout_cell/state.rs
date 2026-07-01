/// The visual state of a grid cell: idle, or actively being edited (pulsing while
/// the key picker is open for it).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum LayoutCellState {
    #[default]
    Idle,
    Editing,
}

/// The visual state of a grid cell: idle, or actively being edited (pulsing while
/// the key picker is open for it).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum LayoutTileState {
    #[default]
    Idle,
    Editing,
}

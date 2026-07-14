#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum LayoutTileState {
    #[default]
    Idle,
    Editing,
}

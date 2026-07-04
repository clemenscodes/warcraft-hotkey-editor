/// A key-capture chip's state: normally bound, or in a binding conflict (which
/// turns it red). Chosen in the hook.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum KeyCaptureCellState {
    #[default]
    Normal,
    Conflict,
}

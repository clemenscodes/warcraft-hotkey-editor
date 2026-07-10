/// A WC3 slot's glow state: idle, actively highlighted (its picker is open, or it
/// is the current drop target), or in a binding conflict. Chosen by the host; the
/// slot only renders the look. The orthogonal `dragging` flag rides alongside on the
/// slot's props, since it can combine with any of these.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SystemSlotState {
    #[default]
    Idle,
    Highlighted,
    Conflict,
}

/// The visual state of a slot button: idle, being edited (its picker open), or in
/// a binding conflict. Chosen in the hook; the button only renders the look.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SlotButtonState {
    #[default]
    Idle,
    Editing,
    Conflict,
}

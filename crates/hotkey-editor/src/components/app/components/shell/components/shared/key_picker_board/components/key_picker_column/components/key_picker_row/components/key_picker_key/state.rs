/// The visual state of a picker key: a free key, the one currently bound here, or a
/// key already taken elsewhere. Chosen from the cell in `logic.rs`; the key only
/// renders the look it is given.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum KeyPickerKeyState {
    #[default]
    Available,
    Current,
    Conflict,
}

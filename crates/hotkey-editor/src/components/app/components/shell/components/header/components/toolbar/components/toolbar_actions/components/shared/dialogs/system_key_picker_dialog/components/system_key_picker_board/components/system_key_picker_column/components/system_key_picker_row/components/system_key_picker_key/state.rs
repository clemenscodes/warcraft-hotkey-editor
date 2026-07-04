/// The visual state of a system-board key: a free key, the one currently bound,
/// or a key already taken by another system hotkey. Chosen in the picker hook; the
/// key only renders the look it is given.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SystemKeyPickerKeyState {
    #[default]
    Normal,
    Current,
    Conflict,
}

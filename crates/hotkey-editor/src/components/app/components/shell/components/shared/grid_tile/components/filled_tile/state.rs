/// The styled state of an occupied tile. Mutually exclusive: a filled slot is an
/// ordinary ability, a built-in command, or the current selection.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum FilledTileState {
    #[default]
    Filled,
    Command,
    Selected,
}

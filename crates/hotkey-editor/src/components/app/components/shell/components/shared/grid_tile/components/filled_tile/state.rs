/// Which background an occupied tile draws. Mutually exclusive and expressed as its
/// own fill component (`AbilityFill` / `CommandFill`), never a class swap: an ability
/// tile wears the panel fill, a built-in command tile the blue fill. Selection is a
/// separate concern (the mounted `SelectionRing`), so it is not a variant here — a
/// selected tile keeps the ability background.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum FilledTileKind {
    #[default]
    Ability,
    Command,
}

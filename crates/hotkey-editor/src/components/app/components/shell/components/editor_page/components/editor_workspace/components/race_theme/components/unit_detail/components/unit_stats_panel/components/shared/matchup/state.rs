/// How a defense fares against an attack (or vice-versa), tinting the cell.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub enum MatchupStrength {
    #[default]
    Neutral,
    Strong,
    Weak,
}

/// The domain decides the strength band (the balance call); this is only the
/// renderer's presentation view of it, dispatching the tinted cell variant.
impl From<warcraft_keybinds::MatchupStrength> for MatchupStrength {
    fn from(strength: warcraft_keybinds::MatchupStrength) -> Self {
        match strength {
            warcraft_keybinds::MatchupStrength::Strong => Self::Strong,
            warcraft_keybinds::MatchupStrength::Neutral => Self::Neutral,
            warcraft_keybinds::MatchupStrength::Weak => Self::Weak,
        }
    }
}

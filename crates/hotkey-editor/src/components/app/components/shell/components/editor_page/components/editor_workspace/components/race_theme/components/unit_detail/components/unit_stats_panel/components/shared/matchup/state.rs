/// How a defense fares against an attack (or vice-versa), tinting the cell.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub enum MatchupStrength {
    #[default]
    Neutral,
    Strong,
    Weak,
}

impl MatchupStrength {
    pub(super) fn data_attribute(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Strong => "strong",
            Self::Weak => "weak",
        }
    }
}

/// The domain decides the strength band (the balance call); this is only the
/// renderer's presentation view of it, carrying the `data-matchup` tint.
impl From<warcraft_keybinds::MatchupStrength> for MatchupStrength {
    fn from(strength: warcraft_keybinds::MatchupStrength) -> Self {
        match strength {
            warcraft_keybinds::MatchupStrength::Strong => Self::Strong,
            warcraft_keybinds::MatchupStrength::Neutral => Self::Neutral,
            warcraft_keybinds::MatchupStrength::Weak => Self::Weak,
        }
    }
}

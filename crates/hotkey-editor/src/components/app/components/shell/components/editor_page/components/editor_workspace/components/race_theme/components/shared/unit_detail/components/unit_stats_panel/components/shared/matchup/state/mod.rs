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
impl From<warcraft_api::MatchupStrength> for MatchupStrength {
    fn from(strength: warcraft_api::MatchupStrength) -> Self {
        match strength {
            warcraft_api::MatchupStrength::Strong => Self::Strong,
            warcraft_api::MatchupStrength::Neutral => Self::Neutral,
            warcraft_api::MatchupStrength::Weak => Self::Weak,
        }
    }
}

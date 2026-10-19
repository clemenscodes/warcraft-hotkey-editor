#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub enum MatchupStrength {
    #[default]
    Neutral,
    Strong,
    Weak,
}

impl From<warcraft_api::MatchupStrength> for MatchupStrength {
    fn from(strength: warcraft_api::MatchupStrength) -> Self {
        match strength {
            warcraft_api::MatchupStrength::Strong => Self::Strong,
            warcraft_api::MatchupStrength::Neutral => Self::Neutral,
            warcraft_api::MatchupStrength::Weak => Self::Weak,
        }
    }
}

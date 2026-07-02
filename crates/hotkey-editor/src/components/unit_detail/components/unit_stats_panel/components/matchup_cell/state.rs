/// How a defense fares against an attack (or vice-versa), tinting the cell.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum MatchupStrength {
    #[default]
    Neutral,
    Strong,
    Weak,
}

impl MatchupStrength {
    pub(super) fn data_attr(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Strong => "strong",
            Self::Weak => "weak",
        }
    }
}

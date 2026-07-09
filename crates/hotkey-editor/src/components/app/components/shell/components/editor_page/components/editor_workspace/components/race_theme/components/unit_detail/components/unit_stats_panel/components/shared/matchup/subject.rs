use std::fmt;
use warcraft_api::{AttackType, DefenseType};

/// What a matchup cell's label names: the attack type (on a defender's row) or the
/// defense type (on an attacker's row). Carrying the domain type — rather than a
/// pre-rendered string — lets the [`MatchupLabel`](super::components::shared::matchup_label::MatchupLabel)
/// leaf render it through the type's own `Display`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchupSubject {
    Attack(AttackType),
    Defense(DefenseType),
}

impl fmt::Display for MatchupSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attack(attack_type) => write!(formatter, "{attack_type}"),
            Self::Defense(defense_type) => write!(formatter, "{defense_type}"),
        }
    }
}

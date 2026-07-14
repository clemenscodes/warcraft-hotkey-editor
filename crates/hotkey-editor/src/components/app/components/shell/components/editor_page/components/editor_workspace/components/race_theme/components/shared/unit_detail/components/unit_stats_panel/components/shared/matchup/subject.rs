use std::fmt;
use warcraft_api::{AttackType, DefenseType};

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

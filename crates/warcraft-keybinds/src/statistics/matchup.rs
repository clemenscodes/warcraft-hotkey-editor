//! How one damage type fares against one defense type — the attack-vs-defense
//! matchup the statistics card shows. Both the raw damage multiplier and the
//! strength band it falls in come from the game's damage table here, in the
//! domain, so the renderer only formats the percentage and never re-derives the
//! balance decision (the classification thresholds live here, not in the UI).

use warcraft_api::{AttackType, DefenseType};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

/// Where a matchup sits relative to full (100%) damage, seen from the attacker's
/// side: `Strong` above 105% damage, `Weak` below 95%, `Neutral` in between.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub enum MatchupStrength {
    Strong,
    #[default]
    Neutral,
    Weak,
}

impl MatchupStrength {
    /// The same matchup seen from the defender's side: an attack that is `Strong`
    /// against a defense leaves that defender in a `Weak` position, and the reverse.
    /// A `Neutral` matchup is neutral from either side.
    pub fn inverted(self) -> Self {
        match self {
            Self::Strong => Self::Weak,
            Self::Neutral => Self::Neutral,
            Self::Weak => Self::Strong,
        }
    }
}

impl From<f32> for MatchupStrength {
    fn from(multiplier: f32) -> Self {
        const STRONG_THRESHOLD: f32 = 1.05;
        const WEAK_THRESHOLD: f32 = 0.95;
        if multiplier > STRONG_THRESHOLD {
            Self::Strong
        } else if multiplier < WEAK_THRESHOLD {
            Self::Weak
        } else {
            Self::Neutral
        }
    }
}

/// A resolved attack-vs-defense matchup: the raw damage multiplier (1.0 is full
/// damage) and the strength band it falls in. The renderer formats the multiplier
/// as a percentage and decides which side's perspective to show; it never
/// classifies the band itself.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Matchup {
    multiplier: f32,
    strength: MatchupStrength,
}

impl Matchup {
    /// Resolve the matchup for an attack type striking a defense type: read the
    /// damage multiplier from the game's gameplay constants and classify its band.
    pub fn resolve(attack_type: AttackType, defense_type: DefenseType) -> Self {
        let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
        let multiplier = effectiveness.against(defense_type);
        let strength = MatchupStrength::from(multiplier);
        Self {
            multiplier,
            strength,
        }
    }

    pub fn multiplier(self) -> f32 {
        self.multiplier
    }

    pub fn strength(self) -> MatchupStrength {
        self.strength
    }
}

#[cfg(test)]
mod tests {
    use super::MatchupStrength;

    #[test]
    fn classifies_strong_above_the_upper_threshold() {
        let strength = MatchupStrength::from(1.5);
        assert_eq!(strength, MatchupStrength::Strong);
    }

    #[test]
    fn classifies_weak_below_the_lower_threshold() {
        let strength = MatchupStrength::from(0.5);
        assert_eq!(strength, MatchupStrength::Weak);
    }

    #[test]
    fn classifies_neutral_at_full_damage() {
        let strength = MatchupStrength::from(1.0);
        assert_eq!(strength, MatchupStrength::Neutral);
    }

    #[test]
    fn thresholds_themselves_are_neutral() {
        let upper = MatchupStrength::from(1.05);
        let lower = MatchupStrength::from(0.95);
        assert_eq!(upper, MatchupStrength::Neutral);
        assert_eq!(lower, MatchupStrength::Neutral);
    }

    #[test]
    fn inverts_the_band_for_the_defender() {
        assert_eq!(MatchupStrength::Strong.inverted(), MatchupStrength::Weak);
        assert_eq!(MatchupStrength::Weak.inverted(), MatchupStrength::Strong);
        assert_eq!(
            MatchupStrength::Neutral.inverted(),
            MatchupStrength::Neutral
        );
    }
}

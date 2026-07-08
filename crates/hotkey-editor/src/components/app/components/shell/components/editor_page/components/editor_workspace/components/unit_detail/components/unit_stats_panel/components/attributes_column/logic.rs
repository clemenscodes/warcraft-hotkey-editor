use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, HeroStatistics};

/// The hero attributes column's figures, shaped out of the body: each attribute's
/// value at the selected level and whether it is the hero's primary attribute. The
/// runtime primary comparison lives here rather than in the component body.
pub(super) struct AttributeFigures {
    pub(super) strength: AttributeStatistic,
    pub(super) strength_is_primary: bool,
    pub(super) agility: AttributeStatistic,
    pub(super) agility_is_primary: bool,
    pub(super) intelligence: AttributeStatistic,
    pub(super) intelligence_is_primary: bool,
}

impl From<&HeroStatistics> for AttributeFigures {
    fn from(hero: &HeroStatistics) -> Self {
        let primary = hero.primary();
        let strength = hero.strength();
        let agility = hero.agility();
        let intelligence = hero.intelligence();
        let strength_is_primary = primary == PrimaryAttribute::Strength;
        let agility_is_primary = primary == PrimaryAttribute::Agility;
        let intelligence_is_primary = primary == PrimaryAttribute::Intelligence;
        Self {
            strength,
            strength_is_primary,
            agility,
            agility_is_primary,
            intelligence,
            intelligence_is_primary,
        }
    }
}

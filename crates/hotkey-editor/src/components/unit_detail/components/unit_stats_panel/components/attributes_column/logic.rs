use super::super::shared::stat_row::StatRowProps;
use super::kinds::{AgilityKind, IntelligenceKind, MarkedAttribute, StrengthKind};
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::HeroStatistics;

/// The three attribute rows' finished props, each marked whether it is the hero's
/// primary attribute. Shaping the runtime primary comparison out of the component
/// body, the way the combat column's `CombatRows` shapes its rows.
pub(super) struct AttributeRows {
    pub(super) strength_row: StatRowProps<StrengthKind>,
    pub(super) agility_row: StatRowProps<AgilityKind>,
    pub(super) intelligence_row: StatRowProps<IntelligenceKind>,
}

impl From<&HeroStatistics> for AttributeRows {
    fn from(hero: &HeroStatistics) -> Self {
        let primary = hero.primary();
        let strength_statistic = hero.strength();
        let agility_statistic = hero.agility();
        let intelligence_statistic = hero.intelligence();
        let strength_is_primary = primary == PrimaryAttribute::Strength;
        let agility_is_primary = primary == PrimaryAttribute::Agility;
        let intelligence_is_primary = primary == PrimaryAttribute::Intelligence;
        let strength = MarkedAttribute::new(strength_statistic, strength_is_primary);
        let agility = MarkedAttribute::new(agility_statistic, agility_is_primary);
        let intelligence = MarkedAttribute::new(intelligence_statistic, intelligence_is_primary);
        let strength_row = StatRowProps::<StrengthKind> { value: strength };
        let agility_row = StatRowProps::<AgilityKind> { value: agility };
        let intelligence_row = StatRowProps::<IntelligenceKind> {
            value: intelligence,
        };
        Self {
            strength_row,
            agility_row,
            intelligence_row,
        }
    }
}

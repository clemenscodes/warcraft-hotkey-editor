use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, HeroStatistics};

/// The attribute rows' input: each of the hero's three attributes at the selected level
/// and whether it is the hero's primary attribute (which drives the row's gold glow).
/// The runtime primary comparison is resolved here rather than in a component body.
#[derive(Props, Clone, PartialEq)]
pub struct AttributeRowsProps {
    pub strength: AttributeStatistic,
    pub strength_is_primary: bool,
    pub agility: AttributeStatistic,
    pub agility_is_primary: bool,
    pub intelligence: AttributeStatistic,
    pub intelligence_is_primary: bool,
}

impl From<&HeroStatistics> for AttributeRowsProps {
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

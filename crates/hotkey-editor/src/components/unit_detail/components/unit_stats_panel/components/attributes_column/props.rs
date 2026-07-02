use super::super::stat_icon_frame::StatIconFrameProps;
use super::components::attribute_row::AttributeRowProps;
use dioxus::prelude::*;

/// The resolved per-level hero attribute figures the column renders.
#[derive(Clone, PartialEq)]
pub struct HeroDisplayData {
    primary_icon: Asset,
    primary_label: String,
    strength_value: u32,
    strength_per_level: f32,
    agility_value: u32,
    agility_per_level: f32,
    intelligence_value: u32,
    intelligence_per_level: f32,
    primary_is_strength: bool,
    primary_is_agility: bool,
    primary_is_intelligence: bool,
}

impl HeroDisplayData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        primary_icon: Asset,
        primary_label: String,
        strength_value: u32,
        strength_per_level: f32,
        agility_value: u32,
        agility_per_level: f32,
        intelligence_value: u32,
        intelligence_per_level: f32,
        primary_is_strength: bool,
        primary_is_agility: bool,
        primary_is_intelligence: bool,
    ) -> Self {
        Self {
            primary_icon,
            primary_label,
            strength_value,
            strength_per_level,
            agility_value,
            agility_per_level,
            intelligence_value,
            intelligence_per_level,
            primary_is_strength,
            primary_is_agility,
            primary_is_intelligence,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AttributesColumnProps {
    pub hero: HeroDisplayData,
}

/// Which attribute an `AttributeRow` shows.
#[derive(Clone, Copy)]
pub(super) enum Attribute {
    Strength,
    Agility,
    Intelligence,
}

impl From<&HeroDisplayData> for StatIconFrameProps {
    fn from(hero: &HeroDisplayData) -> Self {
        let src = hero.primary_icon;
        let alt = format!("{} primary attribute icon", hero.primary_label);
        Self { src, alt }
    }
}

impl From<(&HeroDisplayData, Attribute)> for AttributeRowProps {
    fn from((hero, attribute): (&HeroDisplayData, Attribute)) -> Self {
        match attribute {
            Attribute::Strength => Self {
                label: "Strength",
                value: hero.strength_value,
                per_level: hero.strength_per_level,
                is_primary: hero.primary_is_strength,
            },
            Attribute::Agility => Self {
                label: "Agility",
                value: hero.agility_value,
                per_level: hero.agility_per_level,
                is_primary: hero.primary_is_agility,
            },
            Attribute::Intelligence => Self {
                label: "Intelligence",
                value: hero.intelligence_value,
                per_level: hero.intelligence_per_level,
                is_primary: hero.primary_is_intelligence,
            },
        }
    }
}

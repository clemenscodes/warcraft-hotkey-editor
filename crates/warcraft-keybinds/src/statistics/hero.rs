//! A hero unit's attribute statistics: the three primary attributes at the selected
//! level, each with its per-level growth, and which one is the hero's primary.

use super::values::Gain;
use warcraft_api::PrimaryAttribute;

/// One hero attribute at the selected level: its current value and its per-level
/// growth (shown as the `+x.x` gain).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct AttributeStatistic {
    value: u32,
    per_level: f32,
}

impl AttributeStatistic {
    pub fn new(value: u32, per_level: f32) -> Self {
        Self { value, per_level }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    pub fn per_level(self) -> f32 {
        self.per_level
    }

    /// The attribute's per-level growth as a [`Gain`], the `+x.x` figure the row shows
    /// beside the value.
    pub fn growth(self) -> Gain {
        Gain::new(self.per_level)
    }
}

/// A hero's three attributes at the selected level, and which one is primary (the
/// attribute that also raises attack damage, shown with the glow treatment).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct HeroStatistics {
    strength: AttributeStatistic,
    agility: AttributeStatistic,
    intelligence: AttributeStatistic,
    primary: PrimaryAttribute,
}

impl HeroStatistics {
    pub fn new(
        strength: AttributeStatistic,
        agility: AttributeStatistic,
        intelligence: AttributeStatistic,
        primary: PrimaryAttribute,
    ) -> Self {
        Self {
            strength,
            agility,
            intelligence,
            primary,
        }
    }

    pub fn strength(self) -> AttributeStatistic {
        self.strength
    }

    pub fn agility(self) -> AttributeStatistic {
        self.agility
    }

    pub fn intelligence(self) -> AttributeStatistic {
        self.intelligence
    }

    pub fn primary(self) -> PrimaryAttribute {
        self.primary
    }
}

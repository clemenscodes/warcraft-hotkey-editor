//! The presentation of each stat figure, keyed by its domain type.
//!
//! The value leaves (`StatValue`, `StatGain`) are generic over a domain figure and
//! render it. Per COMPONENTS.md ("Types at the props boundary") the UI owns how it
//! presents a normalized domain
//! value, and `std::fmt::Display` cannot be implemented for these foreign figures in
//! the renderer crate (the orphan rule), so this renderer-local presentation trait is
//! the sanctioned home for the formatting. The domain keeps only the value and answers
//! `is_zero` where a figure can be muted.

use warcraft_api::{
    ArmorFigure as Armor, AttackRange, AttackSpeed, AttributeStatistic, DamagePerSecond,
    DamageRange, EffectiveHitPoints, Evasion, Gain, HitPoints, HitPointsRegen, Mana, ManaRegen,
};
use warcraft_api::{AttackType, DefenseType};

/// A domain figure a stat's value leaf renders. It presents itself as text — the UI's
/// job, since presentation never lives on the domain type — and says whether the leaf
/// should render it muted.
pub trait StatFigure: Copy + PartialEq + 'static {
    /// The display string for this figure, formatted here at the leaf.
    fn display(&self) -> String;

    /// Whether the leaf renders this figure muted. Where "muted" means the value is
    /// zero (a mana of zero, a regeneration of zero), the domain type answers it.
    fn is_muted(&self) -> bool;
}

impl StatFigure for HitPoints {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Mana {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        let mana = *self;
        mana.is_zero()
    }
}

impl StatFigure for Armor {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.0}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for EffectiveHitPoints {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.0}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Evasion {
    fn display(&self) -> String {
        let percent = self.percent();
        format!("{percent:.0}%")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for Gain {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.1}")
    }

    fn is_muted(&self) -> bool {
        let gain = *self;
        gain.is_zero()
    }
}

impl StatFigure for HitPointsRegen {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.2}")
    }

    fn is_muted(&self) -> bool {
        let regen = *self;
        regen.is_zero()
    }
}

impl StatFigure for ManaRegen {
    fn display(&self) -> String {
        let value = self.value();
        format!("+{value:.2}")
    }

    fn is_muted(&self) -> bool {
        let regen = *self;
        regen.is_zero()
    }
}

impl StatFigure for AttributeStatistic {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DamageRange {
    fn display(&self) -> String {
        let minimum = self.minimum();
        let maximum = self.maximum();
        format!("{minimum}\u{2013}{maximum}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackRange {
    fn display(&self) -> String {
        let value = self.value();
        value.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackSpeed {
    fn display(&self) -> String {
        let cooldown_seconds = self.cooldown_seconds();
        format!("{cooldown_seconds:.2}s")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DamagePerSecond {
    fn display(&self) -> String {
        let value = self.value();
        format!("{value:.1}")
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for DefenseType {
    fn display(&self) -> String {
        self.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

impl StatFigure for AttackType {
    fn display(&self) -> String {
        self.to_string()
    }

    fn is_muted(&self) -> bool {
        false
    }
}

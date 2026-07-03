//! A unit's combat statistics as strongly-typed domain values.
//!
//! [`UnitStatistics`] is the single struct the renderer's statistics card receives:
//! every figure is its own wrapper type ([`HitPoints`], [`Armor`], `Mana`, …) so the
//! card renders typed values and never a bag of pre-formatted strings. The derived
//! figures (effective hit points, damage per second) and the hero-level attribute
//! scaling are computed here, in the domain, so the renderer never re-derives a
//! number.

mod attack;
mod hero;
mod matchup;
mod unit_statistics;
mod values;

pub use attack::AttackStatistics;
pub use hero::{AttributeStatistic, HeroStatistics};
pub use matchup::{Matchup, MatchupStrength};
pub use unit_statistics::UnitStatistics;
pub use values::{
    Armor, AttackRange, AttackSpeed, DamagePerSecond, DamageRange, EffectiveHitPoints, Evasion,
    Gain, HitPoints, HitPointsRegen, Mana, ManaRegen,
};

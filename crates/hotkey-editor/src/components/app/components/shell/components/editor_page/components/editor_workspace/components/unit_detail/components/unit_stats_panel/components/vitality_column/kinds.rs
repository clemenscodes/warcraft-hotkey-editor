//! The [`StatRowKind`] markers for the vitality column's rows. Each binds a domain
//! vitality figure to the base `StatRow` at the call site.

use super::super::shared::stat_row::components::regen_qualifier::RegenQualifier;
use super::super::shared::stat_row::components::stat_row_gain::StatRowGain;
use super::super::shared::stat_row::components::stat_row_value::StatRowValue;
use super::super::shared::stat_row::{StatRowKind, StatRowVariant};
use super::data;
use dioxus::prelude::*;
use warcraft_api::RegenType;
use warcraft_keybinds::{HitPoints, HitPointsRegen, Mana, ManaRegen};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct HitPointsKind;

impl StatRowKind for HitPointsKind {
    type Value = HitPoints;

    fn label() -> String {
        data::HIT_POINTS.to_string()
    }

    fn variant() -> StatRowVariant {
        StatRowVariant::Hp
    }

    fn cells(value: HitPoints) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct HitPointsRegenKind;

impl HitPointsRegenKind {
    /// The italic qualifier shown before the regen figure, sourced from `data.rs`;
    /// `None` when the regeneration applies unconditionally.
    fn qualifier(regen_type: RegenType) -> Option<&'static str> {
        match regen_type {
            RegenType::Night => Some(data::AT_NIGHT),
            RegenType::Blight => Some(data::ON_BLIGHT),
            RegenType::Always | RegenType::None => None,
        }
    }
}

impl StatRowKind for HitPointsRegenKind {
    type Value = HitPointsRegen;

    fn label() -> String {
        data::REGENERATION.to_string()
    }

    fn is_regen() -> bool {
        true
    }

    fn cells(value: HitPointsRegen) -> Element {
        let regen_type = value.regen_type();
        let qualifier = Self::qualifier(regen_type);
        rsx! {
            RegenQualifier { text: qualifier }
            StatRowGain { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ManaKind;

impl StatRowKind for ManaKind {
    type Value = Mana;

    fn label() -> String {
        data::MANA.to_string()
    }

    fn variant() -> StatRowVariant {
        StatRowVariant::Mana
    }

    fn cells(value: Mana) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ManaRegenKind;

impl StatRowKind for ManaRegenKind {
    type Value = ManaRegen;

    fn label() -> String {
        data::REGENERATION.to_string()
    }

    fn variant() -> StatRowVariant {
        StatRowVariant::Mana
    }

    fn is_regen() -> bool {
        true
    }

    fn cells(value: ManaRegen) -> Element {
        rsx! {
            StatRowGain { value }
        }
    }
}

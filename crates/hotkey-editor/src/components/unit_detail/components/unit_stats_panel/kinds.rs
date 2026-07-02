//! The [`StatRowKind`] markers for the vitality and defense rows the panel renders
//! inline. Each is a zero-sized type bound to the base `StatRow` at the call site
//! (`StatRow::<HitPointsKind>`); it names its label (from `data.rs`), its colour
//! variant, and renders its value-side from the domain figure through the shared
//! leaves. This is the stat-row counterpart to the grid's `PlainTileKind`.

use super::components::shared::stat_row::components::regen_qualifier::RegenQualifier;
use super::components::shared::stat_row::components::stat_row_gain::StatRowGain;
use super::components::shared::stat_row::components::stat_row_value::StatRowValue;
use super::components::shared::stat_row::{StatRowKind, StatRowVariant};
use super::data;
use dioxus::prelude::*;
use warcraft_api::{DefenseType, RegenType};
use warcraft_keybinds::{
    Armor, EffectiveHitPoints, Evasion, HitPoints, HitPointsRegen, Mana, ManaRegen,
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ArmorKind;

impl StatRowKind for ArmorKind {
    type Value = Armor;

    fn label() -> String {
        data::ARMOR.to_string()
    }

    fn cells(value: Armor) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct DefenseTypeKind;

impl StatRowKind for DefenseTypeKind {
    type Value = DefenseType;

    fn label() -> String {
        data::DEFENSE_TYPE.to_string()
    }

    fn cells(value: DefenseType) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct EffectiveHitPointsKind;

impl StatRowKind for EffectiveHitPointsKind {
    type Value = EffectiveHitPoints;

    fn label() -> String {
        data::EFFECTIVE_HIT_POINTS.to_string()
    }

    fn cells(value: EffectiveHitPoints) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct EvasionKind;

impl StatRowKind for EvasionKind {
    type Value = Evasion;

    fn label() -> String {
        data::EVASION.to_string()
    }

    fn cells(value: Evasion) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

//! The [`StatRowKind`] markers for the defense column's rows. Each binds a domain
//! defense figure to the base `StatRow` at the call site.

use super::super::shared::stat_row::StatRowKind;
use super::super::shared::stat_row::components::stat_row_value::StatRowValue;
use super::data;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

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

//! The [`StatRowKind`] markers for the combat column's rows. Each binds a domain
//! attack figure to the base `StatRow` at the call site.

use super::super::shared::stat_row::StatRowKind;
use super::super::shared::stat_row::components::stat_row_value::StatRowValue;
use super::data;
use dioxus::prelude::*;
use warcraft_api::AttackType;
use warcraft_keybinds::{AttackRange, AttackSpeed, DamagePerSecond, DamageRange};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct DamageKind;

impl StatRowKind for DamageKind {
    type Value = DamageRange;

    fn label() -> String {
        data::DAMAGE.to_string()
    }

    fn cells(value: DamageRange) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct RangeKind;

impl StatRowKind for RangeKind {
    type Value = AttackRange;

    fn label() -> String {
        data::RANGE.to_string()
    }

    fn cells(value: AttackRange) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AttackSpeedKind;

impl StatRowKind for AttackSpeedKind {
    type Value = AttackSpeed;

    fn label() -> String {
        data::ATTACK_SPEED.to_string()
    }

    fn cells(value: AttackSpeed) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct DamagePerSecondKind;

impl StatRowKind for DamagePerSecondKind {
    type Value = DamagePerSecond;

    fn label() -> String {
        data::DAMAGE_PER_SECOND.to_string()
    }

    fn cells(value: DamagePerSecond) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AttackTypeKind;

impl StatRowKind for AttackTypeKind {
    type Value = AttackType;

    fn label() -> String {
        data::ATTACK_TYPE.to_string()
    }

    fn cells(value: AttackType) -> Element {
        rsx! {
            StatRowValue { value }
        }
    }
}

//! The [`StatRowKind`] markers for the hero attributes column. The three attribute
//! rows share one value type and cells shape, differing only in their attribute; each
//! row's label is the attribute's name, which is domain vocabulary owned by
//! `PrimaryAttribute` — so it is sourced from that type's `Display`, never re-typed as
//! a renderer-local string literal. The three `cells` bodies are identical; they are
//! duplicated per kind (duplication over coupling) so the value type stays pure data.

use super::super::shared::stat_row::StatRowKind;
use super::super::shared::stat_row::components::stat_row_gain::StatRowGain;
use super::super::shared::stat_row::components::stat_row_value::StatRowValue;
use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::AttributeStatistic;

/// One hero attribute row's value: the attribute at the selected level, plus whether
/// it is the hero's primary (which glows gold). `is_primary` cannot be a per-type
/// constant — which attribute is primary is runtime hero data — so it rides on the
/// value the row carries. A pure data value: it produces no markup.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MarkedAttribute {
    statistic: AttributeStatistic,
    is_primary: bool,
}

impl MarkedAttribute {
    pub fn new(statistic: AttributeStatistic, is_primary: bool) -> Self {
        Self {
            statistic,
            is_primary,
        }
    }

    fn statistic(self) -> AttributeStatistic {
        self.statistic
    }

    fn is_primary(self) -> bool {
        self.is_primary
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct StrengthKind;

impl StatRowKind for StrengthKind {
    type Value = MarkedAttribute;

    fn label() -> String {
        let attribute = PrimaryAttribute::Strength;
        attribute.to_string()
    }

    fn is_primary(value: &MarkedAttribute) -> bool {
        value.is_primary()
    }

    fn cells(value: MarkedAttribute) -> Element {
        let statistic = value.statistic();
        let growth = statistic.growth();
        rsx! {
            StatRowValue { value: statistic }
            StatRowGain { value: growth }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AgilityKind;

impl StatRowKind for AgilityKind {
    type Value = MarkedAttribute;

    fn label() -> String {
        let attribute = PrimaryAttribute::Agility;
        attribute.to_string()
    }

    fn is_primary(value: &MarkedAttribute) -> bool {
        value.is_primary()
    }

    fn cells(value: MarkedAttribute) -> Element {
        let statistic = value.statistic();
        let growth = statistic.growth();
        rsx! {
            StatRowValue { value: statistic }
            StatRowGain { value: growth }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct IntelligenceKind;

impl StatRowKind for IntelligenceKind {
    type Value = MarkedAttribute;

    fn label() -> String {
        let attribute = PrimaryAttribute::Intelligence;
        attribute.to_string()
    }

    fn is_primary(value: &MarkedAttribute) -> bool {
        value.is_primary()
    }

    fn cells(value: MarkedAttribute) -> Element {
        let statistic = value.statistic();
        let growth = statistic.growth();
        rsx! {
            StatRowValue { value: statistic }
            StatRowGain { value: growth }
        }
    }
}

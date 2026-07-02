pub mod components;
mod hooks;
mod leveled_stats;
mod props;
pub mod stat_icon;
mod style;

use crate::assert_component;
use components::attributes_column::AttributesColumn;
use components::combat_column::CombatColumn;
use components::defense_matchup_row::DefenseMatchupRow;
use components::regen_qualifier::RegenQualifier;
use components::stat_column::{StatColumn, StatColumnKind};
use components::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use components::stat_row::{StatRow, StatRowVariant};
use components::stat_row_gain::StatRowGain;
use components::stat_row_label::StatRowLabel;
use components::stat_row_value::StatRowValue;
use components::stat_rows::StatRows;
use dioxus::prelude::*;
use hooks::use_unit_stats_panel;
pub use props::UnitStatsPanelProps;
use style::CLASS;
assert_component!(UnitStatsPanel);

/// The four stat-category columns (vitality, combat, defense, attributes) in a 2×2
/// grid. Every figure is resolved in the composed hook; the body only places them.
#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelProps) -> Element {
    let model = use_unit_stats_panel(&props);
    let defense_icon = StatIconFrameProps {
        src: model.defense_icon,
        alt: model.defense_icon_alt,
    };
    rsx! {
        div {
            class: CLASS,
            StatColumn {
                kind: StatColumnKind::Vitality,
                if model.has_hp {
                    StatRow {
                        variant: StatRowVariant::Hp,
                        StatRowLabel { text: "Hit Points" }
                        StatRowValue { text: model.display_hp_text }
                    }
                    StatRow {
                        is_regen: true,
                        StatRowLabel { text: "Regeneration" }
                        if model.has_regen {
                            if let Some(qualifier) = model.regen_qualifier {
                                RegenQualifier { text: qualifier }
                            }
                            StatRowGain { text: model.regen_text }
                        } else {
                            StatRowGain { text: "+0.00", is_zero: true }
                        }
                    }
                }
                StatRow {
                    variant: StatRowVariant::Mana,
                    StatRowLabel { text: "Mana" }
                    StatRowValue { text: model.mana_display_text, is_zero: !model.has_mana }
                }
                StatRow {
                    variant: StatRowVariant::Mana,
                    is_regen: true,
                    StatRowLabel { text: "Regeneration" }
                    StatRowGain { text: model.mana_regen_text, is_zero: !model.has_mana_regen }
                }
            }
            if let Some(attack) = model.attack {
                CombatColumn { attack }
            }
            StatColumn {
                kind: StatColumnKind::Defense,
                with_icon: true,
                StatIconFrame { ..defense_icon }
                StatRows {
                    StatRow {
                        StatRowLabel { text: "Armor" }
                        StatRowValue { text: model.armor_text }
                    }
                    StatRow {
                        StatRowLabel { text: "Defense Type" }
                        StatRowValue { text: model.defense_label }
                    }
                    StatRow {
                        StatRowLabel { text: "Effective Hit Points" }
                        StatRowValue { text: model.effective_hit_points_text }
                    }
                    if model.has_evasion {
                        StatRow {
                            StatRowLabel { text: "Evasion" }
                            StatRowValue { text: model.evasion_text }
                        }
                    } else if !model.has_attack {
                        StatRow { "\u{00a0}" }
                    }
                    DefenseMatchupRow { defense_type: model.defense_type }
                }
            }
            if let Some(hero) = model.hero {
                AttributesColumn { hero }
            }
        }
    }
}

pub mod attack_matchup_cell;
pub mod attribute_row;
pub mod attributes_column;
pub mod combat_column;
pub mod damage_matchup_row;
pub mod defense_matchup_cell;
pub mod defense_matchup_row;
mod hooks;
mod leveled_stats;
pub mod matchup_cell;
mod props;
pub mod regen_qualifier;
pub mod stat_column;
pub mod stat_icon;
pub mod stat_icon_frame;
pub mod stat_row;
pub mod stat_row_gain;
pub mod stat_row_label;
pub mod stat_row_value;
pub mod stat_rows;
mod style;

use crate::assert_component;
use attributes_column::AttributesColumn;
use combat_column::CombatColumn;
use defense_matchup_row::DefenseMatchupRow;
use dioxus::prelude::*;
use hooks::use_unit_stats_panel;
pub use props::UnitStatsPanelProps;
use regen_qualifier::RegenQualifier;
use stat_column::{StatColumn, StatColumnKind};
use stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use stat_row::{StatRow, StatRowVariant};
use stat_row_gain::StatRowGain;
use stat_row_label::StatRowLabel;
use stat_row_value::StatRowValue;
use stat_rows::StatRows;
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

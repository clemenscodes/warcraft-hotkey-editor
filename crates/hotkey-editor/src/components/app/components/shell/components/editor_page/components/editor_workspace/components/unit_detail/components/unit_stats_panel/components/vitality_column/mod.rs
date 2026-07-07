mod data;
mod kinds;
mod logic;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_row::StatRow;
use dioxus::prelude::*;
use logic::VitalityRows;
pub use props::VitalityColumnProps;

/// The vitality column: the unit's hit points and mana rows with their regeneration.
/// Always present; every unit has vitality figures.
use tw_macro::assert_component;
assert_component!(VitalityColumn);
#[component]
pub fn VitalityColumn(props: VitalityColumnProps) -> Element {
    let VitalityRows {
        hit_points_row,
        hit_points_regen_row,
        mana_row,
        mana_regen_row,
    } = VitalityRows::from(&props);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Vitality,
            StatRow { ..hit_points_row }
            StatRow { ..hit_points_regen_row }
            StatRow { ..mana_row }
            StatRow { ..mana_regen_row }
        }
    }
}

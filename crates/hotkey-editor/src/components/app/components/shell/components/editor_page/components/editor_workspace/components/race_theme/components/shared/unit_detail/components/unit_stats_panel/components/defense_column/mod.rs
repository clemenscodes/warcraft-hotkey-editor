pub mod components;
mod logic;
mod props;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::defense_rows::{DefenseRows, DefenseRowsProps};
use dioxus::prelude::*;
use logic::DefenseFigures;
pub use props::DefenseColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DefenseColumn);

/// The defense column: the defense-type icon beside its stat rows and the defender's
/// matchup grid, laid into the `defense` grid area. Always present; every unit has
/// defense figures.
#[component]
pub fn DefenseColumn(props: DefenseColumnProps) -> Element {
    let DefenseFigures {
        defense_icon,
        armor,
        defense_type,
        effective_hit_points,
        evasion,
    } = DefenseFigures::from(&props);
    let rows = DefenseRowsProps {
        armor,
        defense_type,
        effective_hit_points,
        evasion,
    };
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..defense_icon }
            DefenseRows { ..rows }
        }
    }
}

pub mod components;
mod props;
mod view;

pub use view::VitalityColumnView;
mod style;

use components::hit_points_regen_row::HitPointsRegenRow;
use components::hit_points_row::HitPointsRow;
use components::mana_regen_row::ManaRegenRow;
use components::mana_row::ManaRow;
use dioxus::prelude::*;
use props::VitalityColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// The vitality column: the unit's hit points and mana rows with their regeneration,
/// laid into the `vitality` grid area. Always present; every unit has vitality
/// figures. It names its four rows directly — each row owns its own look — rather than
/// configuring one shared row through variant flags.
#[component]
pub fn VitalityColumn(props: VitalityColumnProps) -> Element {
    let hit_points = props.hit_points;
    let hit_points_regen = props.hit_points_regen;
    let mana = props.mana;
    let mana_regen = props.mana_regen;
    rsx! {
        div {
            class: CLASS,
            HitPointsRow { value: hit_points }
            HitPointsRegenRow { value: hit_points_regen }
            ManaRow { value: mana }
            ManaRegenRow { value: mana_regen }
        }
    }
}

assert_component!(VitalityColumn);

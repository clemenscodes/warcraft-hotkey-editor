pub mod components;
mod model;
mod view;

pub use view::IslandConflictPanelView;
mod style;

use super::super::presentation::IslandUnitData;
use components::island_conflict_ability_row::IslandConflictAbilityRow;
use components::island_conflict_unit::IslandConflictUnit;
use dioxus::prelude::*;
use model::IslandConflictPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The card surface: the affected unit heading its two clashing abilities. It owns its
/// own bordered, tinted, centered panel element directly.
#[component]
pub fn IslandConflictPanel(props: IslandConflictPanelModel) -> Element {
    let IslandConflictPanelModel {
        unit,
        own_ability,
        shared_ability,
    } = props;
    let IslandUnitData {
        unit_id,
        icon_url,
        name,
    } = unit;
    rsx! {
        div {
            class: CLASS,
            IslandConflictUnit { unit_id, icon_url, name }
            IslandConflictAbilityRow { own_ability, shared_ability }
        }
    }
}

assert_component!(IslandConflictPanel);

pub mod components;
mod props;
mod style;

use components::island_conflict_ability_row::{
    IslandConflictAbilityRow, IslandConflictAbilityRowProps,
};
use components::island_conflict_unit::IslandConflictUnit;
use dioxus::prelude::*;
pub use props::IslandConflictPanelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IslandConflictPanel);

/// The card surface: the affected unit heading its two clashing abilities. It owns its
/// own bordered, tinted, centered panel element directly.
#[component]
pub fn IslandConflictPanel(props: IslandConflictPanelProps) -> Element {
    let ability_row = IslandConflictAbilityRowProps::from(&props);
    let unit = props.unit;
    rsx! {
        div {
            class: CLASS,
            IslandConflictUnit { ..unit }
            IslandConflictAbilityRow { ..ability_row }
        }
    }
}

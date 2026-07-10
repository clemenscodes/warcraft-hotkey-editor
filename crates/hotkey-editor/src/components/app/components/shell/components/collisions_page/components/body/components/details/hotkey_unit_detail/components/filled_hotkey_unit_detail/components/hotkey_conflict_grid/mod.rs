pub mod components;
mod props;
mod style;

use components::hotkey_conflict_card::HotkeyConflictCard;
use dioxus::prelude::*;
use props::HotkeyConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling grid of shared-hotkey conflict cards for the selected unit.
#[component]
pub fn HotkeyConflictGrid(props: HotkeyConflictGridProps) -> Element {
    let conflicts = props.conflicts;
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            for conflict in conflicts {
                HotkeyConflictCard {
                    conflict,
                    unit_id,
                }
            }
        }
    }
}

assert_component!(HotkeyConflictGrid);

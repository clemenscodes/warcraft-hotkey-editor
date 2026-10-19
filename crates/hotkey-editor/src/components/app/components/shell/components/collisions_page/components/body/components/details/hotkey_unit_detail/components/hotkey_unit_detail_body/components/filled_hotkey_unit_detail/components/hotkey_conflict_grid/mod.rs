pub mod components;
mod model;
mod view;

pub use view::HotkeyConflictGridView;
mod style;

use components::hotkey_conflict_card::HotkeyConflictCard;
use dioxus::prelude::*;
use model::HotkeyConflictGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyConflictGrid(props: HotkeyConflictGridModel) -> Element {
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

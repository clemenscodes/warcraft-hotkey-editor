pub mod components;
mod props;
mod style;

use components::hotkey_conflict_card::HotkeyConflictCard;
use dioxus::prelude::*;
pub use props::HotkeyConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyConflictGrid);

/// The scrolling grid of shared-hotkey conflict cards for the selected unit.
#[component]
pub fn HotkeyConflictGrid(props: HotkeyConflictGridProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                HotkeyConflictCard { ..card }
            }
        }
    }
}

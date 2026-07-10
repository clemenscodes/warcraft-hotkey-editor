pub mod components;
mod props;
mod style;

use components::hotkey_conflict_grid::{HotkeyConflictGrid, HotkeyConflictGridProps};
use components::hotkey_detail_header::{HotkeyDetailHeader, HotkeyDetailHeaderProps};
use dioxus::prelude::*;
pub use props::FilledHotkeyUnitDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated shared-hotkey detail pane: the selected unit's header over its
/// shared-hotkey conflict cards.
#[component]
pub fn FilledHotkeyUnitDetail(props: FilledHotkeyUnitDetailProps) -> Element {
    let header = HotkeyDetailHeaderProps::from(&props);
    let grid = HotkeyConflictGridProps::from(&props);
    rsx! {
        section {
            class: CLASS,
            HotkeyDetailHeader { ..header }
            HotkeyConflictGrid { ..grid }
        }
    }
}

assert_component!(FilledHotkeyUnitDetail);

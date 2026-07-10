pub mod components;
mod props;
mod style;

use components::hotkey_conflict_grid::HotkeyConflictGrid;
use components::hotkey_detail_header::HotkeyDetailHeader;
use dioxus::prelude::*;
use props::FilledHotkeyUnitDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated shared-hotkey detail pane: the selected unit's header over its
/// shared-hotkey conflict cards.
#[component]
pub fn FilledHotkeyUnitDetail(props: FilledHotkeyUnitDetailProps) -> Element {
    let unit_view = props.unit_view;
    let unit = unit_view.unit().clone();
    let count = unit_view.collision_count();
    let unit_id = unit.unit_id();
    let conflicts = unit_view.conflicts().to_vec();
    rsx! {
        section {
            class: CLASS,
            HotkeyDetailHeader {
                unit,
                count,
            }
            HotkeyConflictGrid {
                conflicts,
                unit_id,
            }
        }
    }
}

assert_component!(FilledHotkeyUnitDetail);

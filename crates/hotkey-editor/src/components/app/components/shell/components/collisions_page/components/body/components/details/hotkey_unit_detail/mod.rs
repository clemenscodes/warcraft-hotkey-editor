mod components;
mod data;
mod logic;
mod props;
mod style;

use components::hotkey_conflict_grid::{HotkeyConflictGrid, HotkeyConflictGridProps};
use components::hotkey_detail_header::{HotkeyDetailHeader, HotkeyDetailHeaderProps};
use dioxus::prelude::*;
use logic::HotkeyUnitDetailData;
pub use props::HotkeyUnitDetailProps;
use style::DETAIL;
use tw_macro::assert_component;
assert_component!(HotkeyUnitDetail);

/// The shared-hotkey detail pane: the selected unit's header over its conflict cards.
/// It owns its own pane element directly and composes the header and grid children;
/// renders the empty prompt when nothing is selected.
#[component]
pub fn HotkeyUnitDetail(props: HotkeyUnitDetailProps) -> Element {
    let Some(data) = logic::selected(&props) else {
        return rsx! {
            section {
                class: DETAIL,
                "data-empty": true,
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    let HotkeyUnitDetailData {
        unit,
        name,
        unit_id_label,
        count,
        cards,
    } = data;
    let header = HotkeyDetailHeaderProps {
        unit,
        name,
        unit_id_label,
        count,
    };
    let grid = HotkeyConflictGridProps { cards };
    rsx! {
        section {
            class: DETAIL,
            "data-empty": false,
            HotkeyDetailHeader { ..header }
            HotkeyConflictGrid { ..grid }
        }
    }
}

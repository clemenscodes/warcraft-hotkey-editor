mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::hotkey_conflict_card::HotkeyConflictCard;
use dioxus::prelude::*;
use logic::HotkeyUnitDetailData;
pub use props::HotkeyUnitDetailProps;
use style::{DETAIL, GRID, HEADER};
use tw_macro::assert_component;
assert_component!(HotkeyUnitDetail);

/// The shared-hotkey detail pane: the selected unit's header over its conflict cards.
/// It owns its own pane, header, and grid elements directly; renders the empty prompt
/// when nothing is selected.
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
    rsx! {
        section {
            class: DETAIL,
            "data-empty": false,
            header {
                class: HEADER,
                ConflictDetailUnit { ..unit }
                RowMeta {
                    ConflictUnitName { text: name }
                    ConflictObjectId { text: unit_id_label }
                    CollisionCount { count }
                }
            }
            div {
                class: GRID,
                for card in cards {
                    HotkeyConflictCard { ..card }
                }
            }
        }
    }
}

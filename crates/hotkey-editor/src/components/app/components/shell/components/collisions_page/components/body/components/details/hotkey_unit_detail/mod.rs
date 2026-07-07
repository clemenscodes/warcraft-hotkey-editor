mod components;
mod data;
mod logic;
mod props;

use super::detail::{Detail, DetailBody, DetailContent};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::hotkey_detail_unit::HotkeyDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::hotkey_unit_name::HotkeyUnitName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::hotkey_conflict_card::HotkeyConflictCard;
use dioxus::prelude::*;
use logic::selected;
pub use props::HotkeyUnitDetailProps;

/// The hotkey-collision detail extension: builds the selected unit's header and
/// hotkey-conflict cards, fed into the base detail pane.
#[component]
pub fn HotkeyUnitDetail(props: HotkeyUnitDetailProps) -> Element {
    let Some(model) = selected(&props) else {
        let content = DetailContent::Empty(data::EMPTY_PROMPT);
        return rsx! {
            Detail { content }
        };
    };
    let header = rsx! {
        HotkeyDetailUnit { ..model.unit }
        RowMeta {
            HotkeyUnitName { text: model.name }
            ConflictObjectId { text: model.unit_id_label }
            CollisionCount { count: model.count }
        }
    };
    let cards = rsx! {
        for card in model.cards {
            HotkeyConflictCard { ..card }
        }
    };
    let body = DetailBody::new(header, cards);
    let content = DetailContent::Loaded(body);
    rsx! {
        Detail { content }
    }
}

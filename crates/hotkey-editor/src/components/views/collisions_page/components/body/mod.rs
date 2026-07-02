pub mod components;
mod props;

use components::clear_state::ClearState;
use components::content::Content;
use components::empty_state::EmptyState;
use components::hotkey_unit_detail::HotkeyUnitDetail;
use components::hotkey_unit_sidebar::HotkeyUnitSidebar;
use components::island_detail::IslandDetail;
use components::island_sidebar::IslandSidebar;
use components::unit_position_detail::UnitPositionDetail;
use components::unit_position_sidebar::UnitPositionSidebar;
use dioxus::prelude::*;
pub use props::{BodyProps, ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};

/// The active collision content: dispatches the shaped `ContentModel` to
/// the upload prompt, the all-clear state, or the kind's two-pane view. Pure
/// data-driven render — the hook decides which variant, this only places it.
#[component]
pub fn Body(props: BodyProps) -> Element {
    match props.content {
        ContentModel::Empty(state) => {
            rsx! {
                EmptyState { ..state }
            }
        }
        ContentModel::Clear(state) => {
            rsx! {
                ClearState { ..state }
            }
        }
        ContentModel::Positions(pane) => {
            let pane = *pane;
            rsx! {
                Content {
                    collision_kind: pane.collision_kind,
                    count: pane.count,
                    IslandSidebar { ..pane.sidebar }
                    IslandDetail { ..pane.detail }
                }
            }
        }
        ContentModel::Hotkeys(pane) => {
            let pane = *pane;
            rsx! {
                Content {
                    collision_kind: pane.collision_kind,
                    count: pane.count,
                    HotkeyUnitSidebar { ..pane.sidebar }
                    HotkeyUnitDetail { ..pane.detail }
                }
            }
        }
        ContentModel::UnitPositions(pane) => {
            let pane = *pane;
            rsx! {
                Content {
                    collision_kind: pane.collision_kind,
                    count: pane.count,
                    UnitPositionSidebar { ..pane.sidebar }
                    UnitPositionDetail { ..pane.detail }
                }
            }
        }
    }
}

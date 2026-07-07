pub mod components;
mod props;

use components::clear_state::ClearState;
use components::content::Content;
use components::details::hotkey_unit_detail::HotkeyUnitDetail;
use components::details::island_detail::IslandDetail;
use components::details::unit_position_detail::UnitPositionDetail;
use components::empty_state::EmptyState;
use components::sidebars::hotkey_unit_sidebar::HotkeyUnitSidebar;
use components::sidebars::island_sidebar::IslandSidebar;
use components::sidebars::unit_position_sidebar::UnitPositionSidebar;
use dioxus::prelude::*;
pub use props::{BodyProps, ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};

/// The active collision content: dispatches the shaped `ContentModel` to
/// the upload prompt, the all-clear state, or the kind's two-pane view. Pure
/// data-driven render — the hook decides which variant, this only places it.
use tw_macro::assert_component;
assert_component!(Body);
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
            let collision_kind = pane.collision_kind();
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            rsx! {
                Content {
                    collision_kind,
                    count,
                    IslandSidebar { ..sidebar }
                    IslandDetail { ..detail }
                }
            }
        }
        ContentModel::Hotkeys(pane) => {
            let collision_kind = pane.collision_kind();
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            rsx! {
                Content {
                    collision_kind,
                    count,
                    HotkeyUnitSidebar { ..sidebar }
                    HotkeyUnitDetail { ..detail }
                }
            }
        }
        ContentModel::UnitPositions(pane) => {
            let collision_kind = pane.collision_kind();
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            rsx! {
                Content {
                    collision_kind,
                    count,
                    UnitPositionSidebar { ..sidebar }
                    UnitPositionDetail { ..detail }
                }
            }
        }
    }
}

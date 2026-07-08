pub mod components;
mod props;

use components::clear_state::ClearState;
use components::empty_state::EmptyState;
use components::hotkeys_content::{HotkeysContent, HotkeysContentProps};
use components::positions_content::{PositionsContent, PositionsContentProps};
use components::unit_positions_content::{UnitPositionsContent, UnitPositionsContentProps};
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
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            let content = PositionsContentProps {
                count,
                sidebar,
                detail,
            };
            rsx! {
                PositionsContent { ..content }
            }
        }
        ContentModel::Hotkeys(pane) => {
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            let content = HotkeysContentProps {
                count,
                sidebar,
                detail,
            };
            rsx! {
                HotkeysContent { ..content }
            }
        }
        ContentModel::UnitPositions(pane) => {
            let count = pane.count();
            let sidebar = pane.sidebar().clone();
            let detail = pane.detail().clone();
            let content = UnitPositionsContentProps {
                count,
                sidebar,
                detail,
            };
            rsx! {
                UnitPositionsContent { ..content }
            }
        }
    }
}

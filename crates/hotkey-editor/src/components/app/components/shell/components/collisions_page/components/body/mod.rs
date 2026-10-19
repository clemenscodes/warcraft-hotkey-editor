pub mod components;
mod data;
mod model;
mod view;

pub use view::BodyView;

use crate::components::app::components::shell::components::shared::clear_state::ClearState;
use crate::components::app::components::shell::components::shared::empty_state::EmptyState;
use components::hotkeys_content::HotkeysContent;
use components::positions_content::PositionsContent;
use components::unit_positions_content::UnitPositionsContent;
use dioxus::prelude::*;
use model::BodyModel;
use tw_macro::assert_component;
pub use view::{ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};

#[component]
pub fn Body(props: BodyModel) -> Element {
    match props.content {
        ContentModel::Empty(message) => {
            rsx! {
                EmptyState {
                    message,
                }
            }
        }
        ContentModel::Clear => {
            rsx! {
                ClearState {
                    text: data::CLEAR_TEXT,
                }
            }
        }
        ContentModel::Positions(pane) => {
            let islands = pane.islands().to_vec();
            rsx! {
                PositionsContent {
                    islands,
                }
            }
        }
        ContentModel::Hotkeys(pane) => {
            let units = pane.units().to_vec();
            rsx! {
                HotkeysContent {
                    units,
                }
            }
        }
        ContentModel::UnitPositions(pane) => {
            let units = pane.units().to_vec();
            rsx! {
                UnitPositionsContent {
                    units,
                }
            }
        }
    }
}

assert_component!(Body);

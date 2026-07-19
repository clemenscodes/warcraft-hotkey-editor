pub mod components;
mod model;
mod view;

pub use view::CollisionPagerCardsView;

use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::clear_state::ClearState;
use crate::components::app::components::shell::components::collisions_page::components::body::components::empty_state::EmptyState;
use components::hotkey_pager_card_host::HotkeyPagerCardHost;
use components::island_pager_card_host::IslandPagerCardHost;
use components::unit_position_pager_card_host::UnitPositionPagerCardHost;
use dioxus::prelude::*;
use model::CollisionPagerCardsModel;
use tw_macro::assert_component;

#[component]
pub fn CollisionPagerCards(props: CollisionPagerCardsModel) -> Element {
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
                ClearState {}
            }
        }
        ContentModel::Positions(pane) => {
            let islands = pane.islands().to_vec();
            rsx! {
                for island in islands {
                    IslandPagerCardHost {
                        key: "{island.key()}",
                        island,
                    }
                }
            }
        }
        ContentModel::Hotkeys(pane) => {
            let units = pane.units().to_vec();
            rsx! {
                for unit in units {
                    HotkeyPagerCardHost {
                        key: "{unit.key()}",
                        unit,
                    }
                }
            }
        }
        ContentModel::UnitPositions(pane) => {
            let units = pane.units().to_vec();
            rsx! {
                for unit in units {
                    UnitPositionPagerCardHost {
                        key: "{unit.key()}",
                        unit,
                    }
                }
            }
        }
    }
}

assert_component!(CollisionPagerCards);

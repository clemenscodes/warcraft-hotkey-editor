pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::ResolvePagerCardsView;
mod style;

use crate::components::app::components::shell::components::shared::clear_state::ClearState;
use components::move_pager_card_host::MovePagerCardHost;
use components::unresolved_pager_card_host::UnresolvedPagerCardHost;
use dioxus::prelude::*;
use model::ResolvePagerCardsModel;
use presentation::ResolvePagerCardsPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolvePagerCards(props: ResolvePagerCardsModel) -> Element {
    let ResolvePagerCardsPresentation {
        moves,
        unresolved,
        is_clear,
    } = ResolvePagerCardsPresentation::from(&props);
    if is_clear {
        return rsx! {
            div {
                class: CLASS,
                ClearState {
                    text: data::CLEAR_TEXT,
                }
            }
        };
    }
    rsx! {
        div {
            class: CLASS,
            for move_view in moves {
                MovePagerCardHost {
                    key: "move-{move_view.mover().object_id()}",
                    move_view,
                }
            }
            for unresolved_view in unresolved {
                UnresolvedPagerCardHost {
                    key: "unresolved-{unresolved_view.ability().object_id()}",
                    unresolved_view,
                }
            }
        }
    }
}

assert_component!(ResolvePagerCards);

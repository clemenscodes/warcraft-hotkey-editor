pub mod components;
mod model;
mod view;

pub use view::ResolvePagerCardsView;

use crate::components::app::components::shell::components::resolve_page::components::clear_state::ClearState;
use components::move_pager_card_host::MovePagerCardHost;
use components::unresolved_pager_card_host::UnresolvedPagerCardHost;
use dioxus::prelude::*;
use model::ResolvePagerCardsModel;
use tw_macro::assert_component;

#[component]
pub fn ResolvePagerCards(props: ResolvePagerCardsModel) -> Element {
    let section = props.section;
    let unresolved = props.unresolved;
    let moves = match section {
        Some(section) => section.moves().to_vec(),
        None => Vec::new(),
    };
    if moves.is_empty() && unresolved.is_empty() {
        return rsx! {
            ClearState {}
        };
    }
    rsx! {
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

assert_component!(ResolvePagerCards);

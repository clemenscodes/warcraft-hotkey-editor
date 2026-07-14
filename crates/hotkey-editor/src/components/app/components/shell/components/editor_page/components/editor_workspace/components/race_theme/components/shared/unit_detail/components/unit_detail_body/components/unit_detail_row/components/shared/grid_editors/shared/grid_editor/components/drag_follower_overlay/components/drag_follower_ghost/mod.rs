pub mod components;
mod model;
mod presentation;
mod view;

pub use view::DragFollowerGhostView;
mod state;

use components::ability_follower_ghost::AbilityFollowerGhost;
use components::command_follower_ghost::CommandFollowerGhost;
use dioxus::prelude::*;
use model::DragFollowerGhostModel;
pub(crate) use presentation::FollowerPresentation;
use state::GhostState;
use tw_macro::assert_component;

/// The drag-follower ghost. A pure dispatcher: it shows nothing when this grid owns no
/// in-progress drag, and otherwise routes to the per-menu ghost component — the menu the
/// dragged tile came from (an ordinary ability menu or a built-in command menu) is domain
/// data, and each menu's ghost owns its own surface look.
#[component]
pub fn DragFollowerGhost(props: DragFollowerGhostModel) -> Element {
    let Some(presentation) = props.presentation else {
        return rsx! {};
    };
    match presentation.state {
        GhostState::Default => {
            rsx! {
                AbilityFollowerGhost {
                    presentation,
                }
            }
        }
        GhostState::Command => {
            rsx! {
                CommandFollowerGhost {
                    presentation,
                }
            }
        }
    }
}

assert_component!(DragFollowerGhost);

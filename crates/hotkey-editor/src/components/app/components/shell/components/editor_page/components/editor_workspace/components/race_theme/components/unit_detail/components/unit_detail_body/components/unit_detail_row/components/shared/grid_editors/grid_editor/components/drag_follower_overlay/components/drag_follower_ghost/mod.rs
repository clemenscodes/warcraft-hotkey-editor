pub mod components;
mod logic;
mod props;
mod state;

use components::ability_follower_ghost::{AbilityFollowerGhost, AbilityFollowerGhostProps};
use components::command_follower_ghost::{CommandFollowerGhost, CommandFollowerGhostProps};
use dioxus::prelude::*;
pub use props::DragFollowerGhostProps;
use state::GhostState;
use tw_macro::assert_component;
assert_component!(DragFollowerGhost);

/// The drag-follower ghost. A pure dispatcher: it shows nothing when this grid owns no
/// in-progress drag, and otherwise routes to the per-menu ghost component — the menu the
/// dragged tile came from (an ordinary ability menu or a built-in command menu) is domain
/// data, and each menu's ghost owns its own surface look.
#[component]
pub fn DragFollowerGhost(props: DragFollowerGhostProps) -> Element {
    let Some(presentation) = props.presentation else {
        return rsx! {};
    };
    let race_attribute = props.race_attribute;
    match presentation.state {
        GhostState::Default => {
            let ghost = AbilityFollowerGhostProps {
                race_attribute,
                presentation,
            };
            rsx! {
                AbilityFollowerGhost { ..ghost }
            }
        }
        GhostState::Command => {
            let ghost = CommandFollowerGhostProps {
                race_attribute,
                presentation,
            };
            rsx! {
                CommandFollowerGhost { ..ghost }
            }
        }
    }
}

mod model;
mod view;

pub use view::CommandFollowerGhostView;
mod style;

use super::super::presentation::FollowerPresentation;
use super::shared::follower_badge::FollowerBadge;
use super::shared::follower_icon::FollowerIcon;
use dioxus::prelude::*;
use model::CommandFollowerGhostModel;
use style::CLASS;
use tw_macro::assert_component;

/// The drag-follower ghost for a tile lifted from a built-in command menu: the blue
/// command surface, the dragged icon, and its hotkey badge, pinned to the cursor.
#[component]
pub fn CommandFollowerGhost(props: CommandFollowerGhostModel) -> Element {
    let FollowerPresentation {
        style,
        badge_state,
        src,
        alt,
        letter,
        ..
    } = props.presentation;
    rsx! {
        div {
            class: CLASS,
            style,
            FollowerIcon { src, alt }
            FollowerBadge {
                letter,
                state: badge_state,
            }
        }
    }
}

assert_component!(CommandFollowerGhost);

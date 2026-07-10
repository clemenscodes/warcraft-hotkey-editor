mod props;
mod view;

pub use view::AbilityFollowerGhostView;
mod style;

use super::super::logic::FollowerPresentation;
use super::shared::follower_badge::FollowerBadge;
use super::shared::follower_figure::FollowerFigure;
use dioxus::prelude::*;
use props::AbilityFollowerGhostProps;
use style::CLASS;
use tw_macro::assert_component;

/// The drag-follower ghost for a tile lifted from an ordinary ability menu: the panel
/// surface, the dragged icon, and its hotkey badge, pinned to the cursor.
#[component]
pub fn AbilityFollowerGhost(props: AbilityFollowerGhostProps) -> Element {
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
            FollowerFigure { src, alt }
            FollowerBadge {
                letter,
                state: badge_state,
            }
        }
    }
}

assert_component!(AbilityFollowerGhost);

mod model;
mod view;

pub use view::AbilityFollowerGhostView;
mod style;

use super::super::presentation::FollowerPresentation;
use super::shared::follower_badge::FollowerBadge;
use super::shared::follower_icon::FollowerIcon;
use dioxus::prelude::*;
use model::AbilityFollowerGhostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AbilityFollowerGhost(props: AbilityFollowerGhostModel) -> Element {
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
            FollowerIcon {
                src,
                alt,
            }
            FollowerBadge {
                letter,
                state: badge_state,
            }
        }
    }
}

assert_component!(AbilityFollowerGhost);

mod props;
mod style;

use super::shared::follower_badge::{FollowerBadge, FollowerBadgeProps};
use super::shared::follower_figure::{FollowerFigure, FollowerFigureProps};
use dioxus::prelude::*;
pub use props::AbilityFollowerGhostProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AbilityFollowerGhost);

/// The drag-follower ghost for a tile lifted from an ordinary ability menu: the panel
/// surface, the dragged icon, and its hotkey badge, pinned to the cursor.
#[component]
pub fn AbilityFollowerGhost(props: AbilityFollowerGhostProps) -> Element {
    let figure = FollowerFigureProps::from(&props.presentation);
    let badge = FollowerBadgeProps::from(&props.presentation);
    let race = props.race_attribute;
    let style = props.presentation.style;
    rsx! {
        div {
            class: CLASS,
            style,
            "data-race": race,
            FollowerFigure { ..figure }
            FollowerBadge { ..badge }
        }
    }
}

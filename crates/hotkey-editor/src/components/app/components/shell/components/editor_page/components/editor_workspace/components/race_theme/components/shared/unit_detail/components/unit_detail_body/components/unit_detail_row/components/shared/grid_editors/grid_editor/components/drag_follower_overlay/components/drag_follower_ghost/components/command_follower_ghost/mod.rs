mod props;
mod style;

use super::shared::follower_badge::{FollowerBadge, FollowerBadgeProps};
use super::shared::follower_figure::{FollowerFigure, FollowerFigureProps};
use dioxus::prelude::*;
pub use props::CommandFollowerGhostProps;
use style::CLASS;
use tw_macro::assert_component;

/// The drag-follower ghost for a tile lifted from a built-in command menu: the blue
/// command surface, the dragged icon, and its hotkey badge, pinned to the cursor.
#[component]
pub fn CommandFollowerGhost(props: CommandFollowerGhostProps) -> Element {
    let figure = FollowerFigureProps::from(&props.presentation);
    let badge = FollowerBadgeProps::from(&props.presentation);
    let style = props.presentation.style;
    rsx! {
        div {
            class: CLASS,
            style,
            FollowerFigure { ..figure }
            FollowerBadge { ..badge }
        }
    }
}

assert_component!(CommandFollowerGhost);

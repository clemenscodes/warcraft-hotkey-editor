pub mod components;
mod logic;
mod props;
mod state;
mod style;

use components::follower_badge::{FollowerBadge, FollowerBadgeProps};
use components::follower_figure::{FollowerFigure, FollowerFigureProps};
use dioxus::prelude::*;
use logic::FollowerPresentation;
pub use props::DragFollowerGhostProps;
use tw_macro::assert_component;
assert_component!(DragFollowerGhost);

#[component]
pub fn DragFollowerGhost(props: DragFollowerGhostProps) -> Element {
    let Some(presentation) = props.presentation else {
        return rsx! {};
    };
    let race = props.race_attribute;
    let figure = FollowerFigureProps::from(&presentation);
    let badge = FollowerBadgeProps::from(&presentation);
    let FollowerPresentation { state, style, .. } = presentation;
    let class = style::class(state);
    rsx! {
        div { class, style, "data-race": race,
            FollowerFigure { ..figure }
            FollowerBadge { ..badge }
        }
    }
}

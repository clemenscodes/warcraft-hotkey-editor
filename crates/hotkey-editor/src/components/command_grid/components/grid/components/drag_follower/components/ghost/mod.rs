mod components;
mod logic;
mod props;

use dioxus::prelude::*;

use components::{FollowerBadge, FollowerBadgeProps, FollowerFigure, FollowerFigureProps};
use logic::FollowerPresentation;

pub use props::DragFollowerGhostProps;

#[component]
pub fn DragFollowerGhost(props: DragFollowerGhostProps) -> Element {
    let Some(presentation) = props.presentation else {
        return rsx! {};
    };
    let race = props.race_attribute;
    let figure = FollowerFigureProps::from(&presentation);
    let badge = FollowerBadgeProps::from(&presentation);
    let FollowerPresentation { class, style, .. } = presentation;
    rsx! {
        div {
            class,
            style,
            "data-race": race,
            FollowerFigure { ..figure }
            FollowerBadge { ..badge }
        }
    }
}

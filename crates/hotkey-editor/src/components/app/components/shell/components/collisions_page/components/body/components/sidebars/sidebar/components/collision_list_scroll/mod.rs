pub mod components;
mod props;
mod style;

use super::super::ListScrollKind;
use crate::assert_component;
use components::collision_list_track::CollisionListTrack;
use dioxus::prelude::*;
pub use props::CollisionListScrollProps;
use style::CLASS;
assert_component!(CollisionListScroll);

/// The scrolling region of a collision sidebar: a vertical list, or the swipe
/// carousel on small screens. Lays the cards out through the collision track.
#[component]
pub fn CollisionListScroll(props: CollisionListScrollProps) -> Element {
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            CollisionListTrack {
                {children}
            }
        }
    }
}

/// The [`ListScrollKind`] marker binding a collision [`super::super::Sidebar`] to
/// the collision list scroll.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct CollisionScroll;

impl ListScrollKind for CollisionScroll {
    fn scroll(children: Element) -> Element {
        rsx! {
            CollisionListScroll {
                {children}
            }
        }
    }
}

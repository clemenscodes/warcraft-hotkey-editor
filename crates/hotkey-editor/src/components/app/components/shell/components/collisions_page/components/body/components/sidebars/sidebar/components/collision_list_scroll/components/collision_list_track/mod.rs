mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionListTrackProps;
use style::CLASS;
assert_component!(CollisionListTrack);

/// The inner track that lays out the collision cards.
#[component]
pub fn CollisionListTrack(props: CollisionListTrackProps) -> Element {
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            {children}
        }
    }
}

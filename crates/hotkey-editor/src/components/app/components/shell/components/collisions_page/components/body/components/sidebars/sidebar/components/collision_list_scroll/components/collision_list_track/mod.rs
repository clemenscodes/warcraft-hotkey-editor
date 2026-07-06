mod props;
mod style;

use dioxus::prelude::*;
pub use props::CollisionListTrackProps;
use style::CLASS;
use tw_macro::assert_component;
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

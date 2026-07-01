mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionsContentProps;
use style::CLASS;
assert_component!(CollisionsContent);

/// The two-pane collision content: a sidebar column beside the fluid detail pane.
#[component]
pub fn CollisionsContent(props: CollisionsContentProps) -> Element {
    let collision_kind = props.collision_kind;
    let count = props.count;
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "{count}",
            {children}
        }
    }
}

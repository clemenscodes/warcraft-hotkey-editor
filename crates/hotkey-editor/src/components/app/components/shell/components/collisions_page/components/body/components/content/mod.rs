mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ContentProps;
use style::CLASS;
assert_component!(Content);

/// The two-pane collision content: a sidebar column beside the fluid detail pane.
#[component]
pub fn Content(props: ContentProps) -> Element {
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

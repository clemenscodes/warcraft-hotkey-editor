mod props;
mod style;
use dioxus::prelude::*;
pub use props::ContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(Content);

/// The two-pane collision content: a sidebar column beside the fluid detail pane.
#[component]
pub fn Content(props: ContentProps) -> Element {
    let collision_kind = props.collision_kind.kind_param();
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

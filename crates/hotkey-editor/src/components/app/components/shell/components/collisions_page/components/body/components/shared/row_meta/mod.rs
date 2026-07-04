mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::RowMetaProps;
use style::CLASS;
assert_component!(RowMeta);

/// The text column of a collision card.
#[component]
pub fn RowMeta(props: RowMetaProps) -> Element {
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            {children}
        }
    }
}

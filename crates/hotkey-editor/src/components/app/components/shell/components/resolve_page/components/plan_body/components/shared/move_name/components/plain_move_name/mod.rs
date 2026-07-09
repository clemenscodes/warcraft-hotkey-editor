mod props;
mod style;

use dioxus::prelude::*;
pub use props::PlainMoveNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlainMoveName);

/// The non-clickable ability name (no owning unit to link to).
#[component]
pub fn PlainMoveName(props: PlainMoveNameProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

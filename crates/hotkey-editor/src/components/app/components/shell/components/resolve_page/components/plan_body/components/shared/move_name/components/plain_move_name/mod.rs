mod props;
mod view;

pub use view::PlainMoveNameView;
mod style;

use dioxus::prelude::*;
use props::PlainMoveNameProps;
use style::CLASS;
use tw_macro::assert_component;

/// The non-clickable ability name (no owning unit to link to).
#[component]
pub fn PlainMoveName(props: PlainMoveNameProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(PlainMoveName);

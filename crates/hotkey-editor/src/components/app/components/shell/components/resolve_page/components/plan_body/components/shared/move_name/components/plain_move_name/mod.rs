mod model;
mod view;

pub use view::PlainMoveNameView;
mod style;

use dioxus::prelude::*;
use model::PlainMoveNameModel;
use style::CLASS;
use tw_macro::assert_component;

/// The non-clickable ability name (no owning unit to link to).
#[component]
pub fn PlainMoveName(props: PlainMoveNameModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(PlainMoveName);

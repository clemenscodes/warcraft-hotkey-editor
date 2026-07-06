mod props;
mod style;

use dioxus::prelude::*;
pub use props::ConflictObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictObjectId);

/// A unit's object id caption.
#[component]
pub fn ConflictObjectId(props: ConflictObjectIdProps) -> Element {
    let text = props.text;
    rsx! {
        code {
            class: CLASS,
            {text}
        }
    }
}

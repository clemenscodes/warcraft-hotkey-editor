mod props;
mod style;

use dioxus::prelude::*;
use props::ConflictObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;

/// A unit's object id caption.
#[component]
pub fn ConflictObjectId(props: ConflictObjectIdProps) -> Element {
    let object_id = props.object_id;
    rsx! {
        code {
            class: CLASS,
            {object_id.value()}
        }
    }
}

assert_component!(ConflictObjectId);

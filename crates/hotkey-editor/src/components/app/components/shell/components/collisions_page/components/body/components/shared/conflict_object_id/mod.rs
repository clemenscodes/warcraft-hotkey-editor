mod model;
mod view;

pub use view::ConflictObjectIdView;
mod style;

use dioxus::prelude::*;
use model::ConflictObjectIdModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictObjectId(props: ConflictObjectIdModel) -> Element {
    let object_id = props.object_id;
    rsx! {
        code {
            class: CLASS,
            {object_id.value()}
        }
    }
}

assert_component!(ConflictObjectId);

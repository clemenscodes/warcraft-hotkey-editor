mod model;
mod view;

pub use view::AbilityIdView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AbilityIdModel;

/// The object id shown under the name in the override panel.
#[component]
pub fn AbilityId(props: AbilityIdModel) -> Element {
    rsx! {
        code { class: CLASS, {props.object_id.value()} }
    }
}

assert_component!(AbilityId);

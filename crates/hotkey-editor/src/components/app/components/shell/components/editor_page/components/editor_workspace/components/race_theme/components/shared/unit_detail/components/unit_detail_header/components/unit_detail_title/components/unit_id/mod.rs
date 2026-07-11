mod model;
mod view;

pub use view::UnitIdView;
mod style;

use dioxus::prelude::*;
use model::UnitIdModel;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's database id.
#[component]
pub fn UnitId(props: UnitIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(UnitId);

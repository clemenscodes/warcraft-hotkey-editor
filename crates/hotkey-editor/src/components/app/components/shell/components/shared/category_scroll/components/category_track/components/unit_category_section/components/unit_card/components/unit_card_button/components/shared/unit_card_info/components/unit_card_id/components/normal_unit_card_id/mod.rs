mod model;
mod view;

pub use view::NormalUnitCardIdView;
mod style;

use dioxus::prelude::*;
use model::NormalUnitCardIdModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NormalUnitCardId(props: NormalUnitCardIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(NormalUnitCardId);

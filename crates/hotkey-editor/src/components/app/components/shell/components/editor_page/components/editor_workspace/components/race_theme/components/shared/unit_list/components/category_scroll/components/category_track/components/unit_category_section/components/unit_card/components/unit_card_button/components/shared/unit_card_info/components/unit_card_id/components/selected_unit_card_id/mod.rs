mod model;
mod view;

pub use view::SelectedUnitCardIdView;
mod style;

use dioxus::prelude::*;
use model::SelectedUnitCardIdModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SelectedUnitCardId(props: SelectedUnitCardIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(SelectedUnitCardId);

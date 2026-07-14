mod model;
mod view;

pub use view::UnitDetailEmptyView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::UnitDetailEmptyModel;

/// The unit-detail card in its empty / not-found state.
#[component]
pub fn UnitDetailEmpty(props: UnitDetailEmptyModel) -> Element {
    let message = props.message;
    rsx! {
        section {
            class: CLASS,
            {message}
        }
    }
}

assert_component!(UnitDetailEmpty);

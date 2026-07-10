mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::UnitDetailEmptyProps;

/// The unit-detail card in its empty / not-found state.
#[component]
pub fn UnitDetailEmpty(props: UnitDetailEmptyProps) -> Element {
    let message = props.message;
    rsx! {
        section { class: CLASS, {message} }
    }
}

assert_component!(UnitDetailEmpty);

mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::UnitDetailEmptyProps;

assert_component!(UnitDetailEmpty);

/// The unit-detail card in its empty / not-found state.
#[component]
pub fn UnitDetailEmpty(props: UnitDetailEmptyProps) -> Element {
    let message = props.message;
    rsx! {
        section { class: CLASS, {message} }
    }
}

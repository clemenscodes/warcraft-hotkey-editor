mod model;
mod view;

pub use view::PrimaryButtonView;
mod style;

use dioxus::prelude::*;
use model::PrimaryButtonModel;
use style::CLASS;
use tw_macro::assert_component;

/// The primary (affirmative) weight of a WC3 action button. Presentational — the
/// dispatcher builds its props and renders it when the variant is primary.
#[component]
pub fn PrimaryButton(props: PrimaryButtonModel) -> Element {
    let onclick = props.onclick;
    let label = props.label;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}

assert_component!(PrimaryButton);

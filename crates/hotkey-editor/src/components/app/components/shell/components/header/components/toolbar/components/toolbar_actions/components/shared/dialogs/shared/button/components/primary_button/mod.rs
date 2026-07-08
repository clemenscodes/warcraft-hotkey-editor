mod props;
mod style;

use dioxus::prelude::*;
pub use props::PrimaryButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PrimaryButton);

/// The primary (affirmative) weight of a WC3 action button. Presentational — the
/// dispatcher builds its props and renders it when the variant is primary.
#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
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

mod props;
mod view;

pub use view::SecondaryButtonView;
mod style;

use dioxus::prelude::*;
use props::SecondaryButtonProps;
use style::CLASS;
use tw_macro::assert_component;

/// The secondary (dismissive) weight of a WC3 action button. Presentational — the
/// dispatcher builds its props and renders it when the variant is secondary.
#[component]
pub fn SecondaryButton(props: SecondaryButtonProps) -> Element {
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

assert_component!(SecondaryButton);

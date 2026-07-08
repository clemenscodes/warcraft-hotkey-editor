mod props;
mod style;

use dioxus::prelude::*;
pub use props::{ButtonProps, ButtonVariant};
use tw_macro::assert_component;
assert_component!(Button);

/// A WC3 action button in primary or secondary weight. A leaf: it owns the
/// `.button` class family and forwards one click. Callers pick the variant and pass
/// the label text.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = style::class(props.variant);
    let onclick = props.onclick;
    let label = props.label.clone();
    rsx! {
        button { class, r#type: "button", onclick, {label} }
    }
}

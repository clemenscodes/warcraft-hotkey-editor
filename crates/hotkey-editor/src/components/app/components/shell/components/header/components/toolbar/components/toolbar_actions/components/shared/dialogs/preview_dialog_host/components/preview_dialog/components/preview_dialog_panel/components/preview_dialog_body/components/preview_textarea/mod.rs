mod props;
mod style;

use dioxus::prelude::*;
pub use props::PreviewTextareaProps;
use style::CLASS;
use tw_macro::assert_component;

/// The read-only serialized CustomKeys.txt view: monospaced and horizontally
/// scrollable so long lines never wrap.
#[component]
pub fn PreviewTextarea(props: PreviewTextareaProps) -> Element {
    let text = props.text;
    rsx! {
        textarea {
            class: CLASS,
            readonly: true,
            spellcheck: false,
            wrap: "off",
            value: "{text}",
        }
    }
}

assert_component!(PreviewTextarea);

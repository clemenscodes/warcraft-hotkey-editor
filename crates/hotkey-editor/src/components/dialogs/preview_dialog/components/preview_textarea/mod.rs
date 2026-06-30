mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::PreviewTextareaProps;

assert_component!(PreviewTextarea);

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

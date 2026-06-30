mod props;
mod style;

use dioxus::prelude::*;

use style::PREVIEW_TEXTAREA_STYLE_SHEETS;

pub use props::PreviewTextareaProps;

/// The read-only serialized CustomKeys.txt view. Owns `.preview-textarea`,
/// monospaced and horizontally scrollable so long lines never wrap.
#[component]
pub fn PreviewTextarea(props: PreviewTextareaProps) -> Element {
    let text = props.text;
    rsx! {
        for href in PREVIEW_TEXTAREA_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        textarea {
            class: "preview-textarea",
            readonly: true,
            spellcheck: false,
            wrap: "off",
            value: "{text}",
        }
    }
}

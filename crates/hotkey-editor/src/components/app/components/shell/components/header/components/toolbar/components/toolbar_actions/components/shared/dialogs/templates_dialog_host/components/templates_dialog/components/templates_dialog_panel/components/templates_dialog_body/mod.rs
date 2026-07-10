pub mod components;
mod props;
mod style;

use components::template_gallery::{TemplateGallery, TemplateGalleryProps};
use dioxus::prelude::*;
pub use props::TemplatesDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TemplatesDialogBody);

/// The templates dialog's scrolling content region between the header and the panel
/// edge, holding the gallery of bundled template cards.
#[component]
pub fn TemplatesDialogBody(props: TemplatesDialogBodyProps) -> Element {
    let gallery = TemplateGalleryProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            TemplateGallery { ..gallery }
        }
    }
}

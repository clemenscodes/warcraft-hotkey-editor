pub mod components;
mod model;
mod view;

pub use view::TemplatesDialogBodyView;
mod style;

use components::template_gallery::TemplateGallery;
use dioxus::prelude::*;
use model::TemplatesDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The templates dialog's scrolling content region between the header and the panel
/// edge, holding the gallery of bundled template cards.
#[component]
pub fn TemplatesDialogBody(props: TemplatesDialogBodyModel) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            TemplateGallery {
                cards,
            }
        }
    }
}

assert_component!(TemplatesDialogBody);

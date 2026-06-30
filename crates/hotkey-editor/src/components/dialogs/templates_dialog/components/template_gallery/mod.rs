pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::template_card::TemplateCard;
use style::TEMPLATE_GALLERY_STYLE_SHEETS;

pub use props::TemplateGalleryProps;

/// The two-column grid of template cards. Owns `.template-gallery`.
#[component]
pub fn TemplateGallery(props: TemplateGalleryProps) -> Element {
    let cards = props.cards;
    rsx! {
        for href in TEMPLATE_GALLERY_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "template-gallery",
            for card in cards {
                TemplateCard { key: "{card.name}", ..card }
            }
        }
    }
}

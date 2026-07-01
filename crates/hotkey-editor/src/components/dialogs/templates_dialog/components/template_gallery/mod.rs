pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::template_card::TemplateCard;
use style::CLASS;

pub use props::TemplateGalleryProps;

assert_component!(TemplateGallery);

/// The two-column grid of template cards. Owns `.template-gallery`.
#[component]
pub fn TemplateGallery(props: TemplateGalleryProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                TemplateCard { key: "{card.name}", ..card }
            }
        }
    }
}

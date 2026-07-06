pub mod components;
mod props;
mod style;

use components::template_card::TemplateCard;
use dioxus::prelude::*;
pub use props::TemplateGalleryProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TemplateGallery);

/// The two-column grid of template cards. Owns `.template-gallery`.
#[component]
pub fn TemplateGallery(props: TemplateGalleryProps) -> Element {
    let cards = props.cards;
    rsx! {
        div { class: CLASS,
            for card in cards {
                TemplateCard { key: "{card.name}", ..card }
            }
        }
    }
}

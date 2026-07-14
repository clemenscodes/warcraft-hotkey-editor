pub mod components;
mod model;
mod view;

pub use view::TemplateGalleryView;
mod style;

use components::template_card::TemplateCard;
use dioxus::prelude::*;
use model::TemplateGalleryModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplateGallery(props: TemplateGalleryModel) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                TemplateCard {
                    key: "{card.name}",
                    name: card.name.clone(),
                    description: card.description.clone(),
                    resolved: card.resolved.clone(),
                    on_apply: card.on_apply,
                }
            }
        }
    }
}

assert_component!(TemplateGallery);

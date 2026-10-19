pub mod components;
mod model;
mod presentation;
mod view;
pub use view::TemplateCardView;
mod style;

use crate::components::app::components::shell::components::shared::grid_carousel_dots::GridCarouselDots;
use components::template_card_previews::TemplateCardPreviews;
use components::template_card_text::TemplateCardText;
use dioxus::prelude::*;
use model::TemplateCardModel;
use presentation::{PREVIEW_GRID_COUNT, TemplateCardPresentation, use_template_card};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplateCard(props: TemplateCardModel) -> Element {
    let name = props.name.clone();
    let description = props.description.clone();
    let resolved = props.resolved.clone();
    let TemplateCardPresentation {
        onclick,
        onscroll,
        active_grid_index,
    } = use_template_card(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            TemplateCardText {
                name,
                description,
            }
            TemplateCardPreviews {
                resolved,
                onscroll,
            }
            GridCarouselDots {
                grid_count: PREVIEW_GRID_COUNT,
                active_grid_index,
            }
        }
    }
}

assert_component!(TemplateCard);

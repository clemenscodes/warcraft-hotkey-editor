pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::template_card_description::TemplateCardDescription;
use components::template_card_name::TemplateCardName;
use style::TEMPLATE_CARD_TEXT_STYLE_SHEETS;

pub use props::TemplateCardTextProps;

/// The card's name-and-description block. Owns `.template-card-text`.
#[component]
pub fn TemplateCardText(props: TemplateCardTextProps) -> Element {
    let name = props.name.clone();
    let description = props.description.clone();
    rsx! {
        for href in TEMPLATE_CARD_TEXT_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "template-card-text",
            TemplateCardName { {name} }
            TemplateCardDescription { {description} }
        }
    }
}

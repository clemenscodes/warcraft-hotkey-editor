mod props;
mod style;

use dioxus::prelude::*;

use style::TEMPLATE_CARD_DESCRIPTION_STYLE_SHEETS;

pub use props::TemplateCardDescriptionProps;

/// A template card's description line. Owns `.template-card-description`.
#[component]
pub fn TemplateCardDescription(props: TemplateCardDescriptionProps) -> Element {
    let description = props.children.clone();
    rsx! {
        for href in TEMPLATE_CARD_DESCRIPTION_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        p {
            class: "template-card-description",
            {description}
        }
    }
}

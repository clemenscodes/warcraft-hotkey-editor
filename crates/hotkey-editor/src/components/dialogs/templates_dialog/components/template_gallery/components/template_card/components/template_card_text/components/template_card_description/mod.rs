mod style;

use dioxus::prelude::*;

use style::TEMPLATE_CARD_DESCRIPTION_STYLE_SHEETS;

/// A template card's description line. Owns `.template-card-description`.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardDescriptionProps {
    pub children: Element,
}

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

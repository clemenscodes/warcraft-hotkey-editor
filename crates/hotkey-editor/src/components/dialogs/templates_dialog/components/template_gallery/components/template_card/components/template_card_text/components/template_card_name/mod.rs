mod style;

use dioxus::prelude::*;

use style::TEMPLATE_CARD_NAME_STYLE_SHEETS;

/// A template card's name heading. Owns `.template-card-name`.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardNameProps {
    pub children: Element,
}

#[component]
pub fn TemplateCardName(props: TemplateCardNameProps) -> Element {
    let name = props.children.clone();
    rsx! {
        for href in TEMPLATE_CARD_NAME_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        h3 {
            class: "template-card-name",
            {name}
        }
    }
}

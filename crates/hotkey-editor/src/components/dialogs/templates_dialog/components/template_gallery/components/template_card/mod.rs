pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::template_card_previews::{TemplateCardPreviews, TemplateCardPreviewsProps};
use components::template_card_text::{TemplateCardText, TemplateCardTextProps};
use props::TemplateCardPresentation;
use style::TEMPLATE_CARD_STYLE_SHEETS;

pub use props::TemplateCardProps;

/// A clickable template card: its name and description above a preview of the
/// command card and research menu it would apply. Owns `.template-card`.
#[component]
pub fn TemplateCard(props: TemplateCardProps) -> Element {
    let text = TemplateCardTextProps::from(&props);
    let previews = TemplateCardPreviewsProps::from(&props);
    let TemplateCardPresentation { onclick } = TemplateCardPresentation::from(&props);
    rsx! {
        for href in TEMPLATE_CARD_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        button {
            class: "template-card",
            r#type: "button",
            onclick,
            TemplateCardText { ..text }
            TemplateCardPreviews { ..previews }
        }
    }
}

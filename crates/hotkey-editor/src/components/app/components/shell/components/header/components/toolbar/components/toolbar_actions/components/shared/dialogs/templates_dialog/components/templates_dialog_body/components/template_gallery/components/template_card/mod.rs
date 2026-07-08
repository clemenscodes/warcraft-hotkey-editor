pub mod components;
mod props;
mod style;

use components::template_card_previews::{TemplateCardPreviews, TemplateCardPreviewsProps};
use components::template_card_text::{TemplateCardText, TemplateCardTextProps};
use dioxus::prelude::*;
use props::TemplateCardPresentation;
pub use props::TemplateCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TemplateCard);

/// A clickable template card: its name and description above a preview of the
/// command card and research menu it would apply. Owns `.template-card`.
#[component]
pub fn TemplateCard(props: TemplateCardProps) -> Element {
    let text = TemplateCardTextProps::from(&props);
    let previews = TemplateCardPreviewsProps::from(&props);
    let TemplateCardPresentation { onclick } = TemplateCardPresentation::from(&props);
    rsx! {
        button { class: CLASS, r#type: "button", onclick,
            TemplateCardText { ..text }
            TemplateCardPreviews { ..previews }
        }
    }
}

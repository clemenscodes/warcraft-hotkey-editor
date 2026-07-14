pub mod components;
mod model;
mod view;
pub use view::TemplateCardView;
mod style;

use components::template_card_previews::TemplateCardPreviews;
use components::template_card_text::TemplateCardText;
use dioxus::prelude::*;
use model::TemplateCardModel;
use model::TemplateCardPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// A clickable template card: its name and description above a preview of the
/// command card and research menu it would apply. Owns `.template-card`.
#[component]
pub fn TemplateCard(props: TemplateCardModel) -> Element {
    let name = props.name.clone();
    let description = props.description.clone();
    let resolved = props.resolved.clone();
    let TemplateCardPresentation { onclick } = TemplateCardPresentation::from(&props);
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
            }
        }
    }
}

assert_component!(TemplateCard);

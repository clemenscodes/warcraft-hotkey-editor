pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::template_card_description::TemplateCardDescription;
use components::template_card_name::TemplateCardName;
use dioxus::prelude::*;
pub use props::TemplateCardTextProps;
use style::CLASS;
assert_component!(TemplateCardText);

/// The card's name-and-description block. Owns `.template-card-text`.
#[component]
pub fn TemplateCardText(props: TemplateCardTextProps) -> Element {
    let name = props.name.clone();
    let description = props.description.clone();
    rsx! {
        div { class: CLASS,
            TemplateCardName { {name} }
            TemplateCardDescription { {description} }
        }
    }
}

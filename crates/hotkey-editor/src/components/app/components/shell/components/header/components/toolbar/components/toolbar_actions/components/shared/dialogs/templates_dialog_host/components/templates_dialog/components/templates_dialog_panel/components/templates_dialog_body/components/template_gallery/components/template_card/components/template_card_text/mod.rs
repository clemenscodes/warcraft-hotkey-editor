pub mod components;
mod props;
mod view;

pub use view::TemplateCardTextView;
mod style;

use components::template_card_description::TemplateCardDescription;
use components::template_card_name::TemplateCardName;
use dioxus::prelude::*;
use props::TemplateCardTextProps;
use style::CLASS;
use tw_macro::assert_component;

/// The card's name-and-description block. Owns `.template-card-text`.
#[component]
pub fn TemplateCardText(props: TemplateCardTextProps) -> Element {
    let name = props.name.clone();
    let description = props.description.clone();
    rsx! {
        div { class: CLASS,
            TemplateCardName { name }
            TemplateCardDescription { description }
        }
    }
}

assert_component!(TemplateCardText);

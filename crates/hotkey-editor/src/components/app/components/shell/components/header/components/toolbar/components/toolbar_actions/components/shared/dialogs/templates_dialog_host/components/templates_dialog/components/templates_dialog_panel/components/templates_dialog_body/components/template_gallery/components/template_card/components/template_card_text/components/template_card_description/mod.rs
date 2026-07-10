mod props;
mod view;

pub use view::TemplateCardDescriptionView;
mod style;

use dioxus::prelude::*;
use props::TemplateCardDescriptionProps;
use style::CLASS;
use tw_macro::assert_component;

/// A template card's description line. Owns `.template-card-description`.
#[component]
pub fn TemplateCardDescription(props: TemplateCardDescriptionProps) -> Element {
    let description = props.description.clone();
    rsx! {
        p { class: CLASS, {description} }
    }
}

assert_component!(TemplateCardDescription);

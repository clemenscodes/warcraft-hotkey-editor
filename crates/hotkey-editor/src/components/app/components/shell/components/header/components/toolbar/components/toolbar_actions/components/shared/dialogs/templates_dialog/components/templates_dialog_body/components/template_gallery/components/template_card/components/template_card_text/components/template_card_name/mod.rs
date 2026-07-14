mod model;
mod view;

pub use view::TemplateCardNameView;
mod style;

use dioxus::prelude::*;
use model::TemplateCardNameModel;
use style::CLASS;
use tw_macro::assert_component;

/// A template card's name heading. Owns `.template-card-name`.
#[component]
pub fn TemplateCardName(props: TemplateCardNameModel) -> Element {
    let name = props.name.clone();
    rsx! {
        h3 {
            class: CLASS,
            {name}
        }
    }
}

assert_component!(TemplateCardName);

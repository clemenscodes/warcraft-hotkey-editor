mod model;
mod view;

pub use view::ActiveAccentView;
mod style;

use dioxus::prelude::*;
use model::ActiveAccentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveAccent(props: ActiveAccentModel) -> Element {
    let hidden = props.hidden;
    rsx! {
        div {
            class: CLASS,
            aria_hidden: hidden,
        }
    }
}

assert_component!(ActiveAccent);

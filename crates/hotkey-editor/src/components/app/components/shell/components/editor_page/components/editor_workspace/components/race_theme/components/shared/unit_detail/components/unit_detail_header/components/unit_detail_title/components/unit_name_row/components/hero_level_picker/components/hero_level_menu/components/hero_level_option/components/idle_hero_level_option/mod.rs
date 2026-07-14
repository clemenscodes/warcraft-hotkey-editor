mod model;
mod view;

pub use view::IdleHeroLevelOptionView;
mod style;

use dioxus::prelude::*;
use model::IdleHeroLevelOptionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleHeroLevelOption(props: IdleHeroLevelOptionModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}

assert_component!(IdleHeroLevelOption);

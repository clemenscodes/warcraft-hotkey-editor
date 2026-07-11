mod model;
mod view;

pub use view::IdleHeroLevelOptionView;
mod style;

use dioxus::prelude::*;
use model::IdleHeroLevelOptionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The idle hero-level option button. Presentational — the dispatcher renders it.
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

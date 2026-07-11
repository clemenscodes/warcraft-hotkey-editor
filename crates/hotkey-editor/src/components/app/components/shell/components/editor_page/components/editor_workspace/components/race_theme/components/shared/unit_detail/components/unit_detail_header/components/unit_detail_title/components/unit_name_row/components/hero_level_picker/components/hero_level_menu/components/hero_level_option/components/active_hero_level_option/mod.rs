mod model;
mod view;

pub use view::ActiveHeroLevelOptionView;
mod style;

use dioxus::prelude::*;
use model::ActiveHeroLevelOptionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active hero-level option button. Presentational — the dispatcher renders it.
#[component]
pub fn ActiveHeroLevelOption(props: ActiveHeroLevelOptionModel) -> Element {
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

assert_component!(ActiveHeroLevelOption);

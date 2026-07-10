mod props;
mod style;

use dioxus::prelude::*;
use props::ActiveHeroLevelOptionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active hero-level option button. Presentational — the dispatcher renders it.
#[component]
pub fn ActiveHeroLevelOption(props: ActiveHeroLevelOptionProps) -> Element {
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

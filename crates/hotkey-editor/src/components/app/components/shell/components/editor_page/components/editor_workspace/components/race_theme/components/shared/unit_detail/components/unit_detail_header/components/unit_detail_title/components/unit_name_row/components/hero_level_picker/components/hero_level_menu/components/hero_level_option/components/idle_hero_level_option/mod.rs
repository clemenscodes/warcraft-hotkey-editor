mod props;
mod style;

use dioxus::prelude::*;
pub use props::IdleHeroLevelOptionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle hero-level option button. Presentational — the dispatcher renders it.
#[component]
pub fn IdleHeroLevelOption(props: IdleHeroLevelOptionProps) -> Element {
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

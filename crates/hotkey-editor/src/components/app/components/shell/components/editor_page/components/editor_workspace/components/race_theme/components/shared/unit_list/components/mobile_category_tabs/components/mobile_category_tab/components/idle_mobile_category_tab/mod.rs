mod props;
mod style;

use dioxus::prelude::*;
use props::IdleMobileCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle mobile category tab: a muted tab that takes the race accent border on hover
/// (read from the theme's `--race-accent`). Presentational.
#[component]
pub fn IdleMobileCategoryTab(props: IdleMobileCategoryTabProps) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: false,
            onclick,
            {label}
        }
    }
}

assert_component!(IdleMobileCategoryTab);

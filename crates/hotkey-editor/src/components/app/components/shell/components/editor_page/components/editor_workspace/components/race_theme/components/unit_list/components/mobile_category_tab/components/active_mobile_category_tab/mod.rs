mod props;
mod style;

use dioxus::prelude::*;
pub use props::ActiveMobileCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ActiveMobileCategoryTab);

/// The active mobile category tab: the current category, wearing the race accent
/// (border, text, glow read from the theme's `--race-accent`). Presentational.
#[component]
pub fn ActiveMobileCategoryTab(props: ActiveMobileCategoryTabProps) -> Element {
    let label = props.label;
    let kind_attr = props.kind_attr;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: true,
            "data-unit-kind": kind_attr,
            onclick,
            {label}
        }
    }
}

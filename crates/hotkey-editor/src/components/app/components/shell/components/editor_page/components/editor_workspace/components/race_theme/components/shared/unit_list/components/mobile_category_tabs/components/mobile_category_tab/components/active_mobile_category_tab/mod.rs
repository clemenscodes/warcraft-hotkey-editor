mod props;
mod style;

use dioxus::prelude::*;
pub use props::ActiveMobileCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active mobile category tab: the current category, wearing the race accent
/// (border, text, glow read from the theme's `--race-accent`). Presentational.
#[component]
pub fn ActiveMobileCategoryTab(props: ActiveMobileCategoryTabProps) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: true,
            onclick,
            {label}
        }
    }
}

assert_component!(ActiveMobileCategoryTab);
